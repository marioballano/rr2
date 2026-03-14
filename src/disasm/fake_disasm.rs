//! Fake disassembly - making bytes look like instructions since forever

use colored::*;
use rand::Rng;


use crate::core::state::ShellState;
use crate::disasm::instructions::{self, FakeInstruction};
use crate::humor::comments;
use crate::cli::parser;

/// Print disassembly - pd command
pub fn print_disassembly(state: &mut ShellState, args: &[&str]) {
    let count = parser::parse_size(args, 16);
    let addr = state.current_address;

    println!();
    print_disasm_block(addr, count);
}

/// Print function disassembly - pdf command
pub fn print_function_disassembly(state: &mut ShellState) {
    let addr = state.current_address;
    
    // Generate a fake function structure
    let func = crate::analysis::functions::generate_function_info(addr);
    
    println!("{}", "╭─[pdf]─────────────────────────────────────────────────────╮".bright_cyan());
    println!("{}", format!("│ fcn.{:08x}", addr).bright_cyan());
    println!("{}", "╰──────────────────────────────────────────────────────────╯".bright_cyan());
    println!();
    
    // Function header
    println!("{}", format!("/ {} {} @ 0x{:08x}", 
        func.size,
        format!("fcn.{}", func.name).cyan(),
        addr
    ));
    
    println!("{}", comments::get_function_comment(&func.name));
    println!("{}", format!("|   ; args: {}  locals: {}  stack: {}", 
        func.args, func.locals, func.stack_size).dimmed());
    println!();

    // Prologue
    print_prologue(addr);
    
    // Body
    let body_addr = addr + 4;
    let body_size = (func.size / 10).max(5).min(20);
    
    // Add some control flow indicators
    let mut rng = rand::thread_rng();
    let jump_points: Vec<usize> = (0..func.blocks.min(3))
        .map(|_| rng.r#gen_range(0..body_size))
        .collect();
    
    for i in 0..body_size {
        let instr_addr = body_addr + (i as u64 * rng.r#gen_range(2..6));
        let instr = instructions::generate_instruction(&mut rng, false);
        
        // Add flow indicators
        let prefix = if jump_points.contains(&i) {
            if rng.r#gen_bool(0.5) { "├──>" } else { "│  " }
        } else {
            "│  "
        };
        
        print_instruction(instr_addr, &instr, prefix);
    }
    
    // Epilogue
    print_epilogue(addr + func.size as u64 - 10);
    
    println!("{}", "\\".bright_cyan());
}

fn print_prologue(addr: u64) {
    let insts = [
        (addr, "push", "rbp", "; save frame pointer"),
        (addr + 1, "mov", "rbp, rsp", "; establish stack frame"),
        (addr + 4, "sub", "rsp, 0x40", "; allocate local space"),
    ];
    
    for (a, mnemonic, operands, comment) in insts {
        let bytes = generate_fake_bytes(mnemonic);
        println!("│  {:>14}  {:16} {:12} {:18} {}",
            format!("0x{:08x}", a).cyan(),
            bytes.dimmed(),
            mnemonic.bright_white(),
            operands.yellow(),
            comment.bright_black()
        );
    }
}

fn print_epilogue(addr: u64) {
    let insts = [
        (addr, "mov", "rsp, rbp", "; restore stack"),
        (addr + 3, "pop", "rbp", "; restore frame pointer"),  
        (addr + 4, "ret", "", "; return to caller"),
    ];
    
    println!("│");
    for (a, mnemonic, operands, comment) in insts {
        let bytes = generate_fake_bytes(mnemonic);
        println!("│  {:>14}  {:16} {:12} {:18} {}",
            format!("0x{:08x}", a).cyan(),
            bytes.dimmed(),
            mnemonic.bright_white(),
            operands.yellow(),
            comment.bright_black()
        );
    }
}

fn print_disasm_block(start_addr: u64, count: usize) {
    let mut rng = rand::thread_rng();
    let mut addr = start_addr;

    for _ in 0..count {
        let instr = instructions::generate_instruction(&mut rng, true);
        print_instruction(addr, &instr, "");
        addr += instr.size as u64;
    }
}

fn print_instruction(addr: u64, instr: &FakeInstruction, prefix: &str) {
    let comment = comments::get_instruction_comment(&instr.mnemonic, &instr.operands)
        .map(|c| c.to_string())
        .unwrap_or_else(|| {
            if rand::thread_rng().r#gen_bool(0.15) {
                comments::get_random_comment().to_string()
            } else {
                String::new()
            }
        });

    let operands_colored = color_operands(&instr.operands);
    
    println!("{}{:>14}  {:16} {:12} {:24} {}",
        prefix.bright_cyan(),
        format!("0x{:08x}", addr).cyan(),
        instr.bytes.dimmed(),
        instr.mnemonic.bright_white(),
        operands_colored,
        comment.bright_black()
    );
}

fn color_operands(operands: &str) -> String {
    // Color registers yellow, immediates green, memory cyan
    let mut result = String::new();
    
    for part in operands.split(", ") {
        if !result.is_empty() {
            result.push_str(", ");
        }
        
        if part.starts_with("0x") || part.starts_with('-') || part.chars().all(|c| c.is_ascii_digit()) {
            result.push_str(&part.green().to_string());
        } else if part.starts_with('[') || part.starts_with("qword") || part.starts_with("dword") {
            result.push_str(&part.cyan().to_string());
        } else if part.contains("rax") || part.contains("rbx") || part.contains("rcx") ||
                  part.contains("rdx") || part.contains("rsi") || part.contains("rdi") ||
                  part.contains("rsp") || part.contains("rbp") || part.contains("rip") ||
                  part.contains("r8") || part.contains("r9") || part.contains("r10") ||
                  part.contains("eax") || part.contains("ebx") {
            result.push_str(&part.yellow().to_string());
        } else {
            result.push_str(&part.white().to_string());
        }
    }
    
    result
}

fn generate_fake_bytes(mnemonic: &str) -> String {
    let mut rng = rand::thread_rng();
    
    let byte_count = match mnemonic {
        "push" => 1,
        "pop" => 1,
        "ret" => 1,
        "nop" => 1,
        "mov" => rng.r#gen_range(3..7),
        "lea" => rng.r#gen_range(3..7),
        "call" => 5,
        "jmp" => rng.r#gen_range(2..6),
        _ => rng.r#gen_range(2..5),
    };

    (0..byte_count)
        .map(|_| format!("{:02x}", rng.r#gen::<u8>()))
        .collect::<Vec<_>>()
        .join("")
}
