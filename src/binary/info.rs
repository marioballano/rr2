//! Binary info commands - telling you about your binary (mostly truthfully)

use colored::*;
use crate::core::state::ShellState;
use crate::humor::insults;

/// Show general info - i command
pub fn show_info(state: &ShellState) {
    if let Some(ref info) = state.binary_info {
        println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan());
        println!("{}", "║                    BINARY INFORMATION                        ║".cyan());
        println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
        
        let joke = match info.file_type.as_str() {
            t if t.contains("Executable") => "(runnable, allegedly)",
            t if t.contains("Shared") => "(sharing is caring)",
            t if t.contains("DLL") => "(dynamic linking: where bugs go to multiply)",
            t if t.contains("Mach") => "(think different™)",
            t if t.contains("Core") => "(press F to pay respects)",
            _ => "(mysterious)",
        };
        
        println!("║  {:<15} {:40} ║", "Type:".bright_white(), format!("{} {}", info.file_type, joke.dimmed()));
        println!("║  {:<15} {:40} ║", "Architecture:".bright_white(), info.arch);
        println!("║  {:<15} {:40} ║", "Bits:".bright_white(), info.bits.to_string());
        println!("║  {:<15} {:40} ║", "Endian:".bright_white(), info.endian);
        println!("║  {:<15} {:40} ║", "Entry:".bright_white(), format!("0x{:08x}", info.entry_point));
        println!("║  {:<15} {:40} ║", "Size:".bright_white(), format!("{} bytes ({:.2} KB)", info.file_size, info.file_size as f64 / 1024.0));
        println!("║  {:<15} {:40} ║", "Sections:".bright_white(), info.sections.len().to_string());
        println!("║  {:<15} {:40} ║", "Symbols:".bright_white(), 
            if info.is_stripped { "stripped (coward)".red().to_string() } else { info.symbols.len().to_string() });
        println!("║  {:<15} {:40} ║", "Debug Info:".bright_white(), 
            if info.has_debug_info { "yes (how kind)".green().to_string() } else { "no (of course not)".yellow().to_string() });
        println!("║  {:<15} {:40} ║", "PIE:".bright_white(), 
            if info.is_pic { "yes (ASLR enabled)".to_string() } else { "no (fixed addresses, how retro)".to_string() });
        
        println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan());
    } else {
        println!("{}", insults::no_file_loaded());
        println!();
        println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan());
        println!("{}", "║                    VOID INFORMATION                          ║".cyan());
        println!("{}", "╠══════════════════════════════════════════════════════════════╣".cyan());
        println!("║  {:<15} {:40} ║", "Type:".bright_white(), "malloc://512 (imaginary)");
        println!("║  {:<15} {:40} ║", "Architecture:".bright_white(), "x86_64 (assumed)");
        println!("║  {:<15} {:40} ║", "Reality:".bright_white(), "questionable");
        println!("║  {:<15} {:40} ║", "Vibe:".bright_white(), "chaotic neutral");
        println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan());
    }
}

/// Show entry point - ie command
pub fn show_entry(state: &ShellState) {
    if let Some(ref info) = state.binary_info {
        println!("{}", "Entry points:".bright_cyan());
        println!("  {} 0x{:08x} {} {}", 
            "•".green(),
            info.entry_point,
            "entry0".cyan(),
            "(where it all begins)".dimmed()
        );
        
        // Check for interesting entry points
        if info.entry_point == 0xDEADBEEF {
            println!("  {} 0xDEADBEEF? Really? This binary is sus.", "⚠".yellow());
        }
    } else {
        println!("{} 0x{:08x} {} {}", 
            "•".green(),
            0x00400000u64,
            "entry0".cyan(),
            "(fictional entry point)".dimmed()
        );
    }
}

/// Show imports - ii command
pub fn show_imports(state: &ShellState) {
    println!("{}", "Imports (external dependencies):".bright_cyan());
    println!("{:<6} {:<30} {:<20}", "idx", "name", "library");
    println!("{}", "─".repeat(60));
    
    if let Some(ref info) = state.binary_info {
        for (i, imp) in info.imports.iter().enumerate() {
            let warning = match imp.name.as_str() {
                "gets" => " ⚠ (buffer overflow waiting to happen)".red().to_string(),
                "strcpy" => " ⚠ (strcpy? in this economy?)".red().to_string(),
                "sprintf" => " ⚠ (format string paradise)".red().to_string(),
                "system" => " ⚠ (shell injection gateway)".yellow().to_string(),
                "exec" | "execve" => " ⚠ (here be dragons)".yellow().to_string(),
                _ => String::new(),
            };
            
            println!("{:<6} {:<30} {:<20}{}",
                format!("{}", i).dimmed(),
                imp.name.bright_yellow(),
                imp.library.dimmed(),
                warning
            );
        }
        
        if info.imports.is_empty() {
            println!("{}", "  No imports found. Static binary or black magic.".dimmed());
        }
    } else {
        // Fake imports for demonstration
        let fake_imports = [
            ("printf", "libc.so.6", ""),
            ("malloc", "libc.so.6", ""),
            ("free", "libc.so.6", " (memory management optimist)"),
            ("strlen", "libc.so.6", ""),
            ("memcpy", "libc.so.6", " (buffer overflow's best friend)"),
        ];
        
        for (i, (name, lib, note)) in fake_imports.iter().enumerate() {
            println!("{:<6} {:<30} {:<20}{}",
                format!("{}", i).dimmed(),
                name.bright_yellow(),
                lib.dimmed(),
                note.dimmed()
            );
        }
    }
}

/// Show exports - iE command
pub fn show_exports(state: &ShellState) {
    println!("{}", "Exports (what this binary offers the world):".bright_cyan());
    println!("{:<6} {:<18} {:<30}", "idx", "address", "name");
    println!("{}", "─".repeat(60));
    
    if let Some(ref info) = state.binary_info {
        if info.exports.is_empty() {
            println!("{}", "  No exports. This binary keeps its secrets.".dimmed());
        } else {
            for (i, exp) in info.exports.iter().enumerate() {
                println!("{:<6} {:<18} {:<30}",
                    format!("{}", i).dimmed(),
                    format!("0x{:08x}", exp.address).cyan(),
                    exp.name.bright_green()
                );
            }
        }
    } else {
        println!("{}", "  No exports (we're making this up anyway)".dimmed());
    }
}

/// Show symbols - is command
pub fn show_symbols(state: &ShellState) {
    println!("{}", "Symbols (naming things is hard):".bright_cyan());
    println!("{:<6} {:<18} {:<10} {:<8} {}", "idx", "address", "size", "type", "name");
    println!("{}", "─".repeat(70));
    
    if let Some(ref info) = state.binary_info {
        if info.is_stripped {
            println!("{}", "  Binary is stripped. The symbols have been yeeted into the void.".yellow());
            println!("{}", "  (This is why we can't have nice things)".dimmed());
        } else {
            for (i, sym) in info.symbols.iter().take(30).enumerate() {
                println!("{:<6} {:<18} {:<10} {:<8} {}",
                    format!("{}", i).dimmed(),
                    format!("0x{:08x}", sym.address).cyan(),
                    sym.size,
                    sym.sym_type.yellow(),
                    sym.name.bright_white()
                );
            }
            
            if info.symbols.len() > 30 {
                println!("{}", format!("  ... and {} more symbols", info.symbols.len() - 30).dimmed());
            }
        }
    } else {
        println!("{}", "  No symbols in the void. Surprising no one.".dimmed());
    }
}

/// Show sections - iS command
pub fn show_sections(state: &ShellState) {
    println!("{}", "Sections (the binary's anatomy):".bright_cyan());
    println!("{:<6} {:<18} {:<18} {:<8} {}", "idx", "address", "size", "perms", "name");
    println!("{}", "─".repeat(70));
    
    if let Some(ref info) = state.binary_info {
        for (i, sec) in info.sections.iter().enumerate() {
            let perm_colored = colorize_permissions(&sec.permissions);
            let note = match sec.name.as_str() {
                ".text" => " (code lives here)",
                ".data" => " (initialized data)",
                ".bss" => " (uninitialized chaos)",
                ".rodata" => " (strings and constants)",
                ".plt" | ".got" => " (linking magic)",
                ".init" | ".fini" => " (startup/shutdown rituals)",
                s if s.contains("debug") => " (debugging breadcrumbs)",
                _ => "",
            };
            
            println!("{:<6} {:<18} {:<18} {} {}{}",
                format!("{}", i).dimmed(),
                format!("0x{:08x}", sec.address).cyan(),
                format!("0x{:x}", sec.size).dimmed(),
                perm_colored,
                sec.name.bright_white(),
                note.dimmed()
            );
        }
    } else {
        // Fake sections
        let fake_sections = [
            (".text", 0x1000u64, 0x5000u64, "r-x"),
            (".data", 0x6000, 0x1000, "rw-"),
            (".bss", 0x7000, 0x500, "rw-"),
            (".rodata", 0x8000, 0x800, "r--"),
            (".imaginary", 0xDEAD, 0x1337, "rwx"),
        ];
        
        for (i, (name, addr, size, perms)) in fake_sections.iter().enumerate() {
            println!("{:<6} {:<18} {:<18} {} {}",
                format!("{}", i).dimmed(),
                format!("0x{:08x}", addr).cyan(),
                format!("0x{:x}", size).dimmed(),
                colorize_permissions(perms),
                name.bright_white()
            );
        }
    }
}

/// Show headers - ih command
pub fn show_headers(state: &ShellState) {
    println!("{}", "Headers (the binary's ID card):".bright_cyan());
    println!();
    
    if let Some(ref info) = state.binary_info {
        match info.file_type.as_str() {
            t if t.contains("ELF") || t.contains("Executable") || t.contains("Shared") => {
                println!("{}", "ELF Header:".yellow());
                println!("  Magic:   7f 45 4c 46 {}", "(ELF, obviously)".dimmed());
                println!("  Class:   ELF{}", info.bits);
                println!("  Data:    {} endian", info.endian);
                println!("  Machine: {}", info.arch);
                println!("  Entry:   0x{:x}", info.entry_point);
            }
            t if t.contains("PE") || t.contains("DLL") => {
                println!("{}", "PE Header:".yellow());
                println!("  Magic:   4d 5a {}", "(MZ - Mark Zbikowski)".dimmed());
                println!("  Machine: {}", info.arch);
                println!("  Entry:   0x{:x}", info.entry_point);
                println!("  Note:    Windows ¯\\_(ツ)_/¯");
            }
            t if t.contains("Mach") => {
                println!("{}", "Mach-O Header:".yellow());
                println!("  Magic:   feedface {}", "(or feedfacf for 64-bit)".dimmed());
                println!("  CPU:     {}", info.arch);
                println!("  Entry:   0x{:x}", info.entry_point);
                println!("  Note:    🍎");
            }
            _ => {
                println!("{}", "Unknown Header Format:".yellow());
                println!("  Magic:   ?? ?? ?? ??");
                println!("  Type:    {}", info.file_type);
                println!("  Note:    Here be dragons");
            }
        }
    } else {
        println!("{}", "Imaginary Header:".yellow());
        println!("  Magic:   de ad be ef");
        println!("  Type:    Void");
        println!("  Note:    This binary exists only in our hearts");
    }
}

/// Show libraries - il command
pub fn show_libraries(state: &ShellState) {
    println!("{}", "Linked Libraries (the binary's friends):".bright_cyan());
    
    if let Some(ref info) = state.binary_info {
        if info.libraries.is_empty() {
            println!("  {}", "No libraries. This binary has no friends (static linking).".dimmed());
        } else {
            for lib in &info.libraries {
                let note = match lib.as_str() {
                    l if l.contains("libc") => " (the C standard library, everyone's friend)",
                    l if l.contains("libpthread") => " (threading: where race conditions party)",
                    l if l.contains("libm") => " (math: because computers can't count)",
                    l if l.contains("libssl") || l.contains("crypto") => " (security: hopefully implemented correctly)",
                    l if l.contains("libdl") => " (dynamic loading: plugins and chaos)",
                    _ => "",
                };
                
                println!("  {} {}{}", "•".green(), lib.bright_white(), note.dimmed());
            }
        }
    } else {
        println!("  {} libc.so.6 {}", "•".green(), "(imaginary but essential)".dimmed());
        println!("  {} libpthread.so.0 {}", "•".green(), "(for theoretical threads)".dimmed());
    }
}

fn colorize_permissions(perms: &str) -> String {
    let mut result = String::new();
    for c in perms.chars() {
        match c {
            'r' => result.push_str(&"r".green().to_string()),
            'w' => result.push_str(&"w".yellow().to_string()),
            'x' => result.push_str(&"x".red().to_string()),
            '-' => result.push_str(&"-".dimmed().to_string()),
            _ => result.push(c),
        }
    }
    result
}
