//! Hex dump - making bytes look pretty

use colored::*;
use rand::Rng;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::core::state::ShellState;
use crate::cli::parser;

/// Print hex dump - px command
pub fn print_hex(state: &ShellState, args: &[&str]) {
    let count = parser::parse_size(args, 64);
    let addr = state.current_address;
    
    let data = get_bytes(state, addr, count);
    
    println!("{} 0x{:08x}:", "Hex dump @".dimmed(), addr);
    print_hex_block(&data, addr, 16);
}

/// Print hex dump as words - pxw command
pub fn print_hex_words(state: &ShellState, args: &[&str]) {
    let count = parser::parse_size(args, 64);
    let addr = state.current_address;
    
    let data = get_bytes(state, addr, count);
    
    println!("{} 0x{:08x} {} {}:", "Words @".dimmed(), addr, "(4 bytes)".dimmed(), "little-endian".dimmed());
    print_hex_words_block(&data, addr);
}

/// Print hex dump as qwords - pxq command
pub fn print_hex_qwords(state: &ShellState, args: &[&str]) {
    let count = parser::parse_size(args, 64);
    let addr = state.current_address;
    
    let data = get_bytes(state, addr, count);
    
    println!("{} 0x{:08x} {} {}:", "Qwords @".dimmed(), addr, "(8 bytes)".dimmed(), "little-endian".dimmed());
    print_hex_qwords_block(&data, addr);
}

/// Print hex with references - pxr command
pub fn print_hex_refs(state: &ShellState, args: &[&str]) {
    let count = parser::parse_size(args, 64);
    let addr = state.current_address;
    
    let data = get_bytes(state, addr, count);
    
    println!("{} 0x{:08x} {}:", "References @".dimmed(), addr, "(with speculation)".dimmed());
    print_hex_refs_block(&data, addr);
}

fn get_bytes(state: &ShellState, addr: u64, count: usize) -> Vec<u8> {
    // Try to read from actual file
    if let Some(ref path) = state.file_path {
        if let Ok(mut file) = File::open(path) {
            // Try to seek and read
            if file.seek(SeekFrom::Start(addr)).is_ok() {
                let mut buffer = vec![0u8; count];
                if let Ok(bytes_read) = file.read(&mut buffer) {
                    buffer.truncate(bytes_read);
                    return buffer;
                }
            }
        }
    }
    
    // Generate fake data with recognizable patterns
    generate_fake_bytes(addr, count)
}

fn generate_fake_bytes(addr: u64, count: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(count);
    let mut rng = rand::thread_rng();
    
    for i in 0..count {
        // Insert some recognizable patterns
        let offset = addr + i as u64;
        
        let byte = if offset % 256 == 0xDE && i + 3 < count {
            0xDE // Start of DEADBEEF
        } else if offset % 256 == 0xDF && i > 0 {
            0xAD
        } else if offset % 256 == 0xE0 && i > 1 {
            0xBE
        } else if offset % 256 == 0xE1 && i > 2 {
            0xEF
        } else if rng.r#gen_bool(0.05) {
            // Occasional null bytes
            0x00
        } else if rng.r#gen_bool(0.1) {
            // ASCII printable
            rng.r#gen_range(0x20..0x7F)
        } else {
            // Random byte
            rng.r#gen()
        };
        
        data.push(byte);
    }
    
    data
}

fn print_hex_block(data: &[u8], start_addr: u64, bytes_per_line: usize) {
    for (i, chunk) in data.chunks(bytes_per_line).enumerate() {
        let addr = start_addr + (i * bytes_per_line) as u64;
        
        // Address
        print!("{:08x}  ", addr);
        
        // Hex bytes
        for (j, byte) in chunk.iter().enumerate() {
            let byte_str = format!("{:02x}", byte);
            
            // Highlight special values
            let colored = match *byte {
                0x00 => byte_str.dimmed(),
                0xDE | 0xAD | 0xBE | 0xEF => byte_str.bright_red(),
                0xCA | 0xFE | 0xBA => byte_str.bright_magenta(),
                0x90 => byte_str.yellow(), // NOP
                0xCC => byte_str.bright_cyan(), // INT3
                0x20..=0x7E => byte_str.green(), // Printable ASCII
                _ => byte_str.white(),
            };
            
            print!("{} ", colored);
            
            // Add extra space in middle
            if j == 7 {
                print!(" ");
            }
        }
        
        // Padding for incomplete lines
        if chunk.len() < bytes_per_line {
            let missing = bytes_per_line - chunk.len();
            for j in 0..missing {
                print!("   ");
                if chunk.len() + j == 7 {
                    print!(" ");
                }
            }
        }
        
        // ASCII representation
        print!(" |");
        for byte in chunk {
            let ch = if *byte >= 0x20 && *byte < 0x7F {
                *byte as char
            } else {
                '.'
            };
            
            let colored_ch = match *byte {
                0x00 => '.'.to_string().dimmed(),
                0x20..=0x7E => ch.to_string().green(),
                _ => ch.to_string().dimmed(),
            };
            print!("{}", colored_ch);
        }
        println!("|");
    }
    
    // Check for special patterns
    check_special_patterns(data);
}

fn print_hex_words_block(data: &[u8], start_addr: u64) {
    let mut rng = rand::thread_rng();
    
    for (i, chunk) in data.chunks(16).enumerate() {
        let addr = start_addr + (i * 16) as u64;
        print!("0x{:08x}  ", addr);
        
        for word_chunk in chunk.chunks(4) {
            if word_chunk.len() == 4 {
                let word = u32::from_le_bytes([word_chunk[0], word_chunk[1], word_chunk[2], word_chunk[3]]);
                let word_str = format!("0x{:08x}", word);
                
                // Color special words
                let colored = match word {
                    0xDEADBEEF => word_str.bright_red().bold(),
                    0xCAFEBABE => word_str.bright_magenta().bold(),
                    0xBAADF00D => word_str.yellow().bold(),
                    0xFEEDFACE | 0xFEEDFACF => word_str.cyan().bold(),
                    0x00000000 => word_str.dimmed(),
                    0xFFFFFFFF => word_str.dimmed(),
                    _ => word_str.white(),
                };
                
                print!("{} ", colored);
            }
        }
        
        // Random commentary
        if rng.r#gen_bool(0.1) {
            let comments = [
                "; looks suspicious",
                "; probably a pointer",
                "; magic number?",
                "; definitely malware",
                "; TODO: investigate",
            ];
            print!(" {}", comments[rng.r#gen_range(0..comments.len())].dimmed());
        }
        
        println!();
    }
}

fn print_hex_qwords_block(data: &[u8], start_addr: u64) {
    for (i, chunk) in data.chunks(16).enumerate() {
        let addr = start_addr + (i * 16) as u64;
        print!("0x{:08x}  ", addr);
        
        for qword_chunk in chunk.chunks(8) {
            if qword_chunk.len() == 8 {
                let qword = u64::from_le_bytes([
                    qword_chunk[0], qword_chunk[1], qword_chunk[2], qword_chunk[3],
                    qword_chunk[4], qword_chunk[5], qword_chunk[6], qword_chunk[7],
                ]);
                
                let qword_str = format!("0x{:016x}", qword);
                
                let colored = match qword {
                    0xDEADBEEFDEADBEEF => qword_str.bright_red().bold(),
                    0xCAFEBABECAFEBABE => qword_str.bright_magenta().bold(),
                    0x0000000000000000 => qword_str.dimmed(),
                    0xFFFFFFFFFFFFFFFF => qword_str.dimmed(),
                    q if (q >> 48) == 0x7FFF => qword_str.cyan(), // Stack-ish
                    q if (q >> 40) == 0x00007F => qword_str.yellow(), // Library-ish
                    _ => qword_str.white(),
                };
                
                print!("{} ", colored);
            }
        }
        
        println!();
    }
}

fn print_hex_refs_block(data: &[u8], start_addr: u64) {
    let mut rng = rand::thread_rng();
    
    for (i, chunk) in data.chunks(8).enumerate() {
        let addr = start_addr + (i * 8) as u64;
        
        if chunk.len() == 8 {
            let qword = u64::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3],
                chunk[4], chunk[5], chunk[6], chunk[7],
            ]);
            
            print!("0x{:08x}  0x{:016x}", addr, qword);
            
            // Speculate about what this might be
            let speculation = if qword == 0 {
                "  ; NULL (nothing to see here)".dimmed().to_string()
            } else if qword == 0xDEADBEEF || qword == 0xDEADBEEFDEADBEEF {
                "  ; ☠️ DEAD BEEF ☠️".bright_red().to_string()
            } else if qword == 0xCAFEBABE || qword == 0xCAFEBABECAFEBABE {
                "  ; ☕ CAFE BABE ☕ (Java magic)".bright_magenta().to_string()
            } else if (qword >> 48) == 0x7FFF || (qword >> 48) == 0x7FFE {
                format!("  ; -> stack (probably local variable)").cyan().to_string()
            } else if (qword >> 40) == 0x00007F {
                let fake_names = ["libc.so", "ld-linux.so", "libpthread.so", "libm.so"];
                format!("  ; -> {} (library)", fake_names[rng.r#gen_range(0..fake_names.len())]).yellow().to_string()
            } else if (qword >> 40) == 0x000040 {
                format!("  ; -> .text (code reference)").green().to_string()
            } else if qword < 0x1000 {
                format!("  ; small value: {}", qword).dimmed().to_string()
            } else if qword > 0xFFFFFFFF00000000 {
                "  ; negative or high pointer".red().to_string()
            } else if rng.r#gen_bool(0.2) {
                let guesses = [
                    "  ; pointer to something",
                    "  ; heap address (maybe)",
                    "  ; vtable entry (speculation)",
                    "  ; return address (perhaps)",
                    "  ; magic constant (who knows)",
                ];
                guesses[rng.r#gen_range(0..guesses.len())].dimmed().to_string()
            } else {
                String::new()
            };
            
            println!("{}", speculation);
        }
    }
}

fn check_special_patterns(data: &[u8]) {
    // Check for DEADBEEF
    for window in data.windows(4) {
        if window == [0xDE, 0xAD, 0xBE, 0xEF] {
            println!();
            println!("{}", "🥩 DEAD BEEF detected! The forbidden snack! 🥩".bright_red().bold());
        }
        if window == [0xCA, 0xFE, 0xBA, 0xBE] {
            println!();
            println!("{}", "☕ CAFE BABE detected! Java was here! ☕".bright_magenta().bold());
        }
        if window == [0xBA, 0xAD, 0xF0, 0x0D] {
            println!();
            println!("{}", "🍔 BAAD FOOD detected! Uninitialized memory! 🍔".yellow().bold());
        }
    }
}
