//! Binary loader - parsing files with varying degrees of success

use std::fs::File;
use std::io::Read;
use std::path::Path;
use goblin::Object;

#[derive(Clone)]
pub struct BinaryInfo {
    pub file_type: String,
    pub arch: String,
    pub bits: u8,
    pub endian: String,
    pub entry_point: u64,
    pub sections: Vec<SectionInfo>,
    pub symbols: Vec<SymbolInfo>,
    pub imports: Vec<ImportInfo>,
    pub exports: Vec<ExportInfo>,
    pub libraries: Vec<String>,
    pub file_size: usize,
    pub is_stripped: bool,
    pub has_debug_info: bool,
    pub is_pic: bool,
}

#[derive(Clone)]
pub struct SectionInfo {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub permissions: String,
}

#[derive(Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub sym_type: String,
}

#[derive(Clone)]
pub struct ImportInfo {
    pub name: String,
    pub library: String,
}

#[derive(Clone)]
pub struct ExportInfo {
    pub name: String,
    pub address: u64,
}

impl BinaryInfo {
    pub fn load(path: &Path) -> Result<Self, String> {
        let mut file = File::open(path)
            .map_err(|e| format!("Failed to open file: {} (it's hiding from you)", e))?;
        
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file: {} (it doesn't want to be read)", e))?;

        let file_size = buffer.len();

        match Object::parse(&buffer) {
            Ok(Object::Elf(elf)) => Ok(Self::from_elf(elf, file_size)),
            Ok(Object::PE(pe)) => Ok(Self::from_pe(pe, file_size)),
            Ok(Object::Mach(mach)) => Ok(Self::from_mach(mach, file_size)),
            Ok(Object::Archive(_)) => Ok(Self::fake_info("Archive", file_size)),
            Ok(Object::Unknown(_)) => Ok(Self::fake_info("Unknown (probably malware)", file_size)),
            Ok(_) => Ok(Self::fake_info("Something goblin doesn't tell us about", file_size)),
            Err(e) => {
                // Return fake info for unrecognized formats
                Ok(Self::fake_info(&format!("Chaos ({})", e), file_size))
            }
        }
    }

    fn from_elf(elf: goblin::elf::Elf, file_size: usize) -> Self {
        let arch = match elf.header.e_machine {
            goblin::elf::header::EM_X86_64 => "x86_64",
            goblin::elf::header::EM_386 => "x86",
            goblin::elf::header::EM_ARM => "ARM",
            goblin::elf::header::EM_AARCH64 => "ARM64",
            goblin::elf::header::EM_RISCV => "RISC-V",
            goblin::elf::header::EM_MIPS => "MIPS",
            goblin::elf::header::EM_PPC64 => "PowerPC64",
            _ => "Unknown (exotic)",
        }.to_string();

        let bits = if elf.is_64 { 64 } else { 32 };
        
        let file_type = match elf.header.e_type {
            goblin::elf::header::ET_EXEC => "Executable",
            goblin::elf::header::ET_DYN => "Shared Object (PIE/Library)",
            goblin::elf::header::ET_REL => "Relocatable",
            goblin::elf::header::ET_CORE => "Core Dump (someone had a bad day)",
            _ => "Mystery",
        }.to_string();

        let sections: Vec<SectionInfo> = elf.section_headers.iter()
            .filter_map(|sh| {
                elf.shdr_strtab.get_at(sh.sh_name).map(|name| {
                    SectionInfo {
                        name: name.to_string(),
                        address: sh.sh_addr,
                        size: sh.sh_size,
                        permissions: format!("{}{}{}",
                            if sh.sh_flags & 0x4 != 0 { "r" } else { "-" },
                            if sh.sh_flags & 0x1 != 0 { "w" } else { "-" },
                            if sh.sh_flags & 0x2 != 0 { "x" } else { "-" },
                        ),
                    }
                })
            })
            .collect();

        let symbols: Vec<SymbolInfo> = elf.syms.iter()
            .filter_map(|sym| {
                elf.strtab.get_at(sym.st_name).map(|name| {
                    SymbolInfo {
                        name: name.to_string(),
                        address: sym.st_value,
                        size: sym.st_size,
                        sym_type: match sym.st_type() {
                            0 => "NOTYPE",
                            1 => "OBJECT",
                            2 => "FUNC",
                            3 => "SECTION",
                            4 => "FILE",
                            _ => "OTHER",
                        }.to_string(),
                    }
                })
            })
            .filter(|s| !s.name.is_empty())
            .collect();

        let imports: Vec<ImportInfo> = elf.dynsyms.iter()
            .filter(|sym| sym.is_import())
            .filter_map(|sym| {
                elf.dynstrtab.get_at(sym.st_name).map(|name| {
                    ImportInfo {
                        name: name.to_string(),
                        library: "libc.so.6".to_string(), // Simplified
                    }
                })
            })
            .filter(|i| !i.name.is_empty())
            .collect();

        let exports: Vec<ExportInfo> = elf.dynsyms.iter()
            .filter(|sym| !sym.is_import() && sym.st_value != 0)
            .filter_map(|sym| {
                elf.dynstrtab.get_at(sym.st_name).map(|name| {
                    ExportInfo {
                        name: name.to_string(),
                        address: sym.st_value,
                    }
                })
            })
            .filter(|e| !e.name.is_empty())
            .collect();

        let libraries: Vec<String> = elf.libraries.iter()
            .map(|s| s.to_string())
            .collect();

        let is_stripped = symbols.len() < 10;
        let has_debug_info = sections.iter().any(|s| s.name.starts_with(".debug"));
        let is_pic = elf.header.e_type == goblin::elf::header::ET_DYN;

        BinaryInfo {
            file_type,
            arch,
            bits,
            endian: if elf.little_endian { "little" } else { "big" }.to_string(),
            entry_point: elf.entry,
            sections,
            symbols,
            imports,
            exports,
            libraries,
            file_size,
            is_stripped,
            has_debug_info,
            is_pic,
        }
    }

    fn from_pe(pe: goblin::pe::PE, file_size: usize) -> Self {
        let bits = if pe.is_64 { 64 } else { 32 };
        
        let sections: Vec<SectionInfo> = pe.sections.iter()
            .map(|s| {
                SectionInfo {
                    name: String::from_utf8_lossy(&s.name).trim_end_matches('\0').to_string(),
                    address: s.virtual_address as u64,
                    size: s.virtual_size as u64,
                    permissions: format!("{}{}{}",
                        if s.characteristics & 0x40000000 != 0 { "r" } else { "-" },
                        if s.characteristics & 0x80000000 != 0 { "w" } else { "-" },
                        if s.characteristics & 0x20000000 != 0 { "x" } else { "-" },
                    ),
                }
            })
            .collect();

        let imports: Vec<ImportInfo> = pe.imports.iter()
            .map(|imp| {
                ImportInfo {
                    name: imp.name.to_string(),
                    library: imp.dll.to_string(),
                }
            })
            .collect();

        let exports: Vec<ExportInfo> = pe.exports.iter()
            .filter_map(|exp| {
                exp.name.map(|name| ExportInfo {
                    name: name.to_string(),
                    address: exp.rva as u64,
                })
            })
            .collect();

        let libraries: Vec<String> = pe.imports.iter()
            .map(|imp| imp.dll.to_string())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        BinaryInfo {
            file_type: if pe.is_lib { "DLL" } else { "PE Executable" }.to_string(),
            arch: if pe.is_64 { "x86_64" } else { "x86" }.to_string(),
            bits,
            endian: "little".to_string(),
            entry_point: pe.entry as u64,
            sections,
            symbols: Vec::new(), // PE symbol parsing is complex
            imports,
            exports,
            libraries,
            file_size,
            is_stripped: true, // Usually
            has_debug_info: false,
            is_pic: pe.is_64, // Simplified assumption
        }
    }

    fn from_mach(mach: goblin::mach::Mach, file_size: usize) -> Self {
        match mach {
            goblin::mach::Mach::Binary(macho) => {
                let bits = if macho.is_64 { 64 } else { 32 };
                
                BinaryInfo {
                    file_type: "Mach-O".to_string(),
                    arch: match macho.header.cputype() {
                        7 => "x86",
                        0x01000007 => "x86_64",
                        12 => "ARM",
                        0x0100000c => "ARM64",
                        _ => "Unknown",
                    }.to_string(),
                    bits,
                    endian: if macho.little_endian { "little" } else { "big" }.to_string(),
                    entry_point: macho.entry,
                    sections: Vec::new(), // Could expand this
                    symbols: Vec::new(),
                    imports: Vec::new(),
                    exports: Vec::new(),
                    libraries: macho.libs.iter().map(|s| s.to_string()).collect(),
                    file_size,
                    is_stripped: true,
                    has_debug_info: false,
                    is_pic: true,
                }
            }
            goblin::mach::Mach::Fat(fat) => {
                BinaryInfo {
                    file_type: format!("Fat Mach-O ({} architectures)", fat.narches),
                    arch: "Universal".to_string(),
                    bits: 64,
                    endian: "varies".to_string(),
                    entry_point: 0,
                    sections: Vec::new(),
                    symbols: Vec::new(),
                    imports: Vec::new(),
                    exports: Vec::new(),
                    libraries: Vec::new(),
                    file_size,
                    is_stripped: true,
                    has_debug_info: false,
                    is_pic: true,
                }
            }
        }
    }

    fn fake_info(format_name: &str, file_size: usize) -> Self {
        BinaryInfo {
            file_type: format_name.to_string(),
            arch: "Unknown (use your imagination)".to_string(),
            bits: 64,
            endian: "Schrödinger's endian".to_string(),
            entry_point: 0xDEADBEEF,
            sections: vec![
                SectionInfo { name: ".text".to_string(), address: 0x1000, size: 0x5000, permissions: "r-x".to_string() },
                SectionInfo { name: ".data".to_string(), address: 0x6000, size: 0x1000, permissions: "rw-".to_string() },
                SectionInfo { name: ".bss".to_string(), address: 0x7000, size: 0x500, permissions: "rw-".to_string() },
                SectionInfo { name: ".rodata".to_string(), address: 0x8000, size: 0x800, permissions: "r--".to_string() },
            ],
            symbols: Vec::new(),
            imports: vec![
                ImportInfo { name: "printf".to_string(), library: "libc".to_string() },
                ImportInfo { name: "malloc".to_string(), library: "libc".to_string() },
                ImportInfo { name: "free".to_string(), library: "libc".to_string() },
            ],
            exports: Vec::new(),
            libraries: vec!["libc.so".to_string()],
            file_size,
            is_stripped: true,
            has_debug_info: false,
            is_pic: false,
        }
    }
}
