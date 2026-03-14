//! Strings - finding text in the binary wilderness

use colored::*;
use rand::Rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::core::state::ShellState;
use crate::cli::parser;

/// Print string at current address - ps command
pub fn print_string(state: &ShellState, args: &[&str]) {
    let addr = state.current_address;
    let max_len = parser::parse_size(args, 256);
    
    let string = get_string_at(state, addr, max_len);
    
    println!("{} 0x{:08x}:", "String @".dimmed(), addr);
    println!("  {} (len: {})", format!("\"{}\"", string).green(), string.len());
}

/// Print zero-terminated string - psz command
pub fn print_string_zero(state: &ShellState, args: &[&str]) {
    let addr = state.current_address;
    let max_len = parser::parse_size(args, 1024);
    
    let string = get_string_at(state, addr, max_len);
    
    println!("{} 0x{:08x}:", "Z-String @".dimmed(), addr);
    println!("  {} + \\0", format!("\"{}\"", string).green());
}

/// Print Pascal string - psp command
pub fn print_string_pascal(state: &ShellState, _args: &[&str]) {
    let addr = state.current_address;
    
    println!("{} 0x{:08x}:", "Pascal String @".dimmed(), addr);
    println!("  {} (Pascal? In this economy?)", "\"Hello\"".green());
    println!("  {}", "Note: Nobody uses Pascal strings anymore. This is a joke.".dimmed());
}

/// List strings in data sections - iz command
pub fn list_strings(state: &ShellState) {
    println!("{}", "Strings in data sections:".bright_cyan());
    println!("{:<6} {:<18} {:<8} {:<6} {}", "idx", "address", "section", "length", "string");
    println!("{}", "─".repeat(80));
    
    let strings = get_strings(state, false);
    
    for (i, (addr, section, string)) in strings.iter().enumerate() {
        let truncated = if string.len() > 50 {
            format!("{}...", &string[..47])
        } else {
            string.clone()
        };
        
        // Add commentary for funny strings
        let commentary = get_string_commentary(string);
        
        println!("{:<6} {:<18} {:<8} {:<6} {} {}",
            format!("{}", i).dimmed(),
            format!("0x{:08x}", addr).cyan(),
            section.yellow(),
            string.len(),
            format!("\"{}\"", truncated).green(),
            commentary.dimmed()
        );
    }
    
    println!();
    println!("{}", format!("Found {} strings (some may be lies)", strings.len()).dimmed());
}

/// List ALL strings - izz command
pub fn list_all_strings(state: &ShellState) {
    println!("{}", "ALL strings (chaos mode enabled):".bright_yellow());
    println!("{}", "Warning: This includes garbage, false positives, and existential dread.".dimmed());
    println!();
    
    let strings = get_strings(state, true);
    
    for (i, (addr, section, string)) in strings.iter().take(50).enumerate() {
        let truncated = if string.len() > 60 {
            format!("{}...", &string[..57])
        } else {
            string.clone()
        };
        
        println!("{:<4} 0x{:08x} [{}] \"{}\"",
            i,
            addr,
            section.yellow(),
            truncated.green()
        );
    }
    
    if strings.len() > 50 {
        println!();
        println!("{}", format!("... and {} more strings (too many to show)", strings.len() - 50).dimmed());
    }
}

fn get_string_at(state: &ShellState, addr: u64, max_len: usize) -> String {
    // Try to read from actual file
    if let Some(ref path) = state.file_path {
        if let Ok(mut file) = File::open(path) {
            if file.seek(SeekFrom::Start(addr)).is_ok() {
                let mut buffer = vec![0u8; max_len];
                if let Ok(bytes_read) = file.read(&mut buffer) {
                    buffer.truncate(bytes_read);
                    
                    // Find null terminator
                    let len = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
                    
                    // Convert to string, replacing non-printable
                    return buffer[..len].iter()
                        .map(|&b| if b >= 0x20 && b < 0x7F { b as char } else { '.' })
                        .collect();
                }
            }
        }
    }
    
    // Generate fake string
    generate_fake_string()
}

fn get_strings(state: &ShellState, all: bool) -> Vec<(u64, String, String)> {
    let mut strings = Vec::new();
    
    // Try to extract real strings from file
    if let Some(ref path) = state.file_path {
        if let Ok(mut file) = File::open(path) {
            let mut buffer = Vec::new();
            if file.read_to_end(&mut buffer).is_ok() {
                extract_strings_from_buffer(&buffer, &mut strings);
            }
        }
    }
    
    // If no real strings or too few, add fake ones
    if strings.len() < 10 {
        add_fake_strings(&mut strings, all);
    }
    
    strings
}

fn extract_strings_from_buffer(buffer: &[u8], strings: &mut Vec<(u64, String, String)>) {
    let min_len = 4;
    let mut current_string = String::new();
    let mut string_start = 0;
    
    for (i, &byte) in buffer.iter().enumerate() {
        if byte >= 0x20 && byte < 0x7F {
            if current_string.is_empty() {
                string_start = i;
            }
            current_string.push(byte as char);
        } else {
            if current_string.len() >= min_len {
                let section = if string_start < 0x1000 {
                    ".rodata"
                } else if string_start < 0x5000 {
                    ".text"
                } else {
                    ".data"
                };
                
                strings.push((
                    string_start as u64,
                    section.to_string(),
                    current_string.clone()
                ));
            }
            current_string.clear();
            
            // Limit results
            if strings.len() >= 100 {
                break;
            }
        }
    }
}

fn add_fake_strings(strings: &mut Vec<(u64, String, String)>, include_garbage: bool) {
    let mut rng = rand::thread_rng();
    let base_addr = 0x00402000;
    
    // Realistic strings
    let realistic = [
        ("Usage: %s [options] <file>", ".rodata"),
        ("Error: %s", ".rodata"),
        ("malloc failed", ".rodata"),
        ("/dev/null", ".rodata"),
        ("password", ".rodata"),
        ("admin", ".rodata"),
        ("root", ".rodata"),
        ("http://", ".rodata"),
        ("https://", ".rodata"),
        ("LICENSE", ".rodata"),
        ("TODO: fix this", ".rodata"),
        ("DEBUG", ".rodata"),
        ("Copyright (C) 2024", ".rodata"),
        ("Version 1.0", ".rodata"),
        ("/etc/passwd", ".rodata"),
        ("/bin/sh", ".rodata"),
        ("ASLR defeated lol", ".rodata"),
    ];
    
    // Funny strings
    let funny = [
        ("// this should never happen", ".rodata"),
        ("TODO: remove before production", ".rodata"),
        ("password123", ".rodata"),
        ("hunter2", ".rodata"),
        ("if you can read this, the decompiler works", ".rodata"),
        ("flag{not_the_real_flag}", ".rodata"),
        ("INSERT_LICENSE_KEY_HERE", ".rodata"),
        ("FIXME: security vulnerability below", ".rodata"),
        ("Please don't reverse engineer this", ".rodata"),
        ("This code is proprietary (lol)", ".rodata"),
        ("Written by an intern at 3am", ".rodata"),
        ("¯\\_(ツ)_/¯", ".rodata"),
        ("undefined behavior goes brrr", ".rodata"),
        ("Stack smashing detected, yeet", ".rodata"),
        ("We've been trying to reach you about your car's extended warranty", ".rodata"),
        ("All work and no play makes Jack a dull boy", ".rodata"),
    ];
    
    // Garbage strings (for izz)
    let garbage = [
        ("AAAA", ".text"),
        ("????", ".text"),
        ("    ", ".bss"),
        ("!@#$", ".data"),
        ("0000", ".bss"),
        ("XXXX", ".text"),
    ];
    
    let offset = strings.len();
    
    // Add realistic strings
    for (i, (s, section)) in realistic.iter().enumerate() {
        let addr = base_addr + (offset + i) as u64 * 0x10 + rng.r#gen_range(0..8);
        strings.push((addr, section.to_string(), s.to_string()));
    }
    
    // Add some funny strings
    let funny_count = if include_garbage { 10 } else { 5 };
    for i in 0..funny_count {
        let (s, section) = funny.choose(&mut rng).unwrap();
        let addr = base_addr + 0x500 + i as u64 * 0x20 + rng.r#gen_range(0..16);
        strings.push((addr, section.to_string(), s.to_string()));
    }
    
    // Add garbage for izz
    if include_garbage {
        for (s, section) in garbage.iter() {
            let addr = rng.r#gen_range(0x401000..0x410000);
            strings.push((addr, section.to_string(), s.to_string()));
        }
    }
    
    // Shuffle to make it look more realistic
    strings.shuffle(&mut rng);
}

fn get_string_commentary(string: &str) -> String {
    let s = string.to_lowercase();
    
    if s.contains("password") || s.contains("passwd") {
        return "⚠ credentials?!".to_string();
    }
    if s.contains("todo") || s.contains("fixme") {
        return "📝 left by developer".to_string();
    }
    if s.contains("http") {
        return "🌐 URL".to_string();
    }
    if s.contains("error") || s.contains("fail") {
        return "❌ error string".to_string();
    }
    if s.contains("/dev/") || s.contains("/etc/") || s.contains("/bin/") {
        return "📁 path".to_string();
    }
    if s.contains("flag{") || s.contains("ctf{") {
        return "🚩 CTF flag?!".to_string();
    }
    if s.contains("copyright") || s.contains("license") {
        return "©️ legal".to_string();
    }
    if s.contains("debug") {
        return "🐛 debug".to_string();
    }
    if s.contains("%s") || s.contains("%d") || s.contains("%x") {
        return "📋 format string".to_string();
    }
    
    String::new()
}

fn generate_fake_string() -> String {
    let mut rng = rand::thread_rng();
    
    let strings = [
        "Hello, World!",
        "Segmentation fault (core dumped)",
        "Error: something went wrong",
        "TODO: implement actual functionality",
        "This string is definitely real",
        "password: hunter2",
        "undefined behavior detected",
        "Stack cookies are delicious",
        "malloc: corrupted heap",
        "Bus error (core dumped)",
    ];
    
    strings.choose(&mut rng).unwrap_or(&"???").to_string()
}
