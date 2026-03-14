//! Visual mode - ASCII art pretending to be a GUI

use colored::*;
use rand::Rng;

use crate::core::state::ShellState;

/// Enter visual mode - V command
pub fn enter_visual_mode(state: &mut ShellState) {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║                       VISUAL MODE                                ║".bright_cyan());
    println!("{}", "║              'Visual' is a strong word                           ║".bright_cyan().dimmed());
    println!("{}", "╚══════════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
    
    let addr = state.current_address;
    let file_name = state.file_name();
    
    // Header bar
    println!("{}", format!(
        "┌─[{}]─[{}]─[0x{:08x}]─────────────────────────────────┐",
        file_name.cyan(),
        if state.write_mode { "W".red() } else { "R".green() },
        addr
    ));
    
    // Fake disassembly view
    print_visual_disasm(addr);
    
    // Status bar
    println!("{}", "├────────────────────────────────────────────────────────────────────┤");
    println!("{}", format!(
        "│ {}  {}  {}  {}  {}  {} │",
        "h/j/k/l:nav".dimmed(),
        "p:print".dimmed(),
        "V:graph".dimmed(),
        "::cmd".dimmed(),
        "?:help".dimmed(),
        "q:quit".dimmed()
    ));
    println!("{}", "└────────────────────────────────────────────────────────────────────┘");
    
    println!();
    println!("{}", "Visual mode exited. (It was fun while it lasted, which was 0.3 seconds)".yellow());
    println!("{}", "Note: Real visual mode requires a TUI library. This is a preview.".dimmed());
}

fn print_visual_disasm(addr: u64) {
    let mut rng = rand::thread_rng();
    
    let instructions = [
        ("55", "push", "rbp"),
        ("4889e5", "mov", "rbp, rsp"),
        ("4883ec40", "sub", "rsp, 0x40"),
        ("897dfc", "mov", "[rbp-0x4], edi"),
        ("488975f0", "mov", "[rbp-0x10], rsi"),
        ("488b45f0", "mov", "rax, [rbp-0x10]"),
        ("4889c7", "mov", "rdi, rax"),
        ("e8000000", "call", "sym.mystery"),
        ("8945ec", "mov", "[rbp-0x14], eax"),
        ("837dec00", "cmp", "dword [rbp-0x14], 0"),
        ("7510", "jne", "0x{:x}"),
        ("b800000000", "mov", "eax, 0"),
        ("c9", "leave", ""),
        ("c3", "ret", ""),
    ];
    
    for i in 0..10 {
        let (bytes, mnemonic, operands) = instructions[i % instructions.len()];
        let current_addr = addr + i as u64 * 3;
        
        let cursor = if i == 0 { ">" } else { " " };
        let marker = if rng.r#gen_bool(0.2) { "@@" } else { "  " };
        
        let line = format!(
            "│{}{} 0x{:08x}  {:12} {:8} {:24} │",
            cursor.bright_green(),
            marker.cyan(),
            current_addr,
            bytes,
            mnemonic,
            operands
        );
        
        if i == 0 {
            println!("{}", line.on_bright_black());
        } else {
            println!("{}", line);
        }
    }
}

/// Mini visual mode - v command
pub fn mini_visual(state: &mut ShellState) {
    println!();
    let addr = state.current_address;
    
    println!("{}", "┌─[ Mini Visual ]───────────────┐".cyan());
    
    // Show a small hex view
    let mut rng = rand::thread_rng();
    
    for i in 0..4 {
        let row_addr = addr + i * 8;
        let bytes: String = (0..8)
            .map(|_| format!("{:02x}", rng.r#gen::<u8>()))
            .collect::<Vec<_>>()
            .join(" ");
        
        println!("│ {:08x}: {} │", row_addr, bytes.dimmed());
    }
    
    println!("{}", "└───────────────────────────────┘".cyan());
    println!("{}", "(That's all folks! Use 'V' for the full experience)".dimmed());
}
