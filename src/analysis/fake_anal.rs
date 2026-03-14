//! Fake analysis - the art of pretending to understand binaries

use colored::*;
use rand::Rng;
use std::{thread, time::Duration};

use crate::core::state::ShellState;
use crate::humor::insults;

/// Analyze all - the aaa command
pub fn analyze_all(state: &mut ShellState) {
    if !state.has_file() {
        println!("{}", insults::no_file_loaded());
        println!("{}", "(Analyzing the void instead...)".dimmed());
    }

    println!("{}", insults::analysis_warning());
    println!();

    let stages = [
        ("Scanning for functions", "afr"),
        ("Detecting basic blocks", "afbr"),
        ("Resolving cross-references", "axt"),
        ("Analyzing stack frames", "afsr"),
        ("Processing symbols", "afs"),
        ("Extracting strings", "izzr"),
        ("Computing cyclomatic complexity", "afcc"),
        ("Divining the programmer's intent", "ai"),
        ("Consulting the ancient scrolls", "ar2"),
        ("Detecting undefined behavior", "aub"),
        ("Measuring technical debt", "atd"),
        ("Finding bugs", "afb"),
    ];

    let mut rng = rand::thread_rng();
    let total_time: u64 = rng.r#gen_range(800..2500);
    let per_stage = total_time / stages.len() as u64;

    for (name, code) in stages.iter() {
        print!("[{}] {}...", code.cyan(), name);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        // Simulate work
        thread::sleep(Duration::from_millis(per_stage + rng.r#gen_range(0..200)));
        
        // Random results
        let result = match *code {
            "afr" => {
                let count = rng.r#gen_range(20..100);
                state.functions_detected = count;
                format!("found {} functions", count)
            }
            "axt" => {
                let count = rng.r#gen_range(50..500);
                state.xrefs_resolved = count;
                format!("{} xrefs resolved", count)
            }
            "izzr" => {
                let count = rng.r#gen_range(30..200);
                state.strings_found = count;
                format!("{} strings (mostly profanity)", count)
            }
            "afcc" => format!("average: {} (concerning)", rng.r#gen_range(10..50)),
            "ai" => "unclear (as usual)".to_string(),
            "ar2" => "they were no help".to_string(),
            "aub" => format!("{} instances (rookie numbers)", rng.r#gen_range(5..20)),
            "atd" => format!("{} years accumulated", rng.r#gen_range(10..50)),
            "afb" => "yes".to_string(),
            _ => "done".to_string(),
        };
        
        println!(" {}", result.green());
    }

    println!();
    state.analysis_done = true;

    // Summary
    let func_comment = match state.functions_detected {
        0..=20 => "Suspiciously few. Is this even a real binary?",
        21..=50 => "Reasonable amount. The programmer showed restraint.",
        51..=80 => "Quite a few. Someone was productive.",
        _ => "Many functions. This binary has seen some things.",
    };

    println!("{}", "═".repeat(60).bright_cyan());
    println!("{}", "ANALYSIS SUMMARY".bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_cyan());
    println!("  Functions detected: {} {}", 
        state.functions_detected.to_string().bright_green(),
        format!("({})", func_comment).dimmed()
    );
    println!("  Xrefs resolved:     {} {}", 
        state.xrefs_resolved.to_string().bright_green(),
        "(it's complicated)".dimmed()
    );
    println!("  Strings found:      {} {}", 
        state.strings_found.to_string().bright_green(),
        "(some unprintable)".dimmed()
    );
    println!("{}", "═".repeat(60).bright_cyan());
    println!();
    println!("{}", "Analysis complete! Results may vary. Dramatically.".yellow().italic());
}

/// Basic analysis - aa command
pub fn analyze_basic(state: &mut ShellState) {
    if !state.has_file() {
        println!("{}", insults::no_file_loaded());
    }

    println!("{}", "Running basic analysis (the lazy version)...".dimmed());
    
    let mut rng = rand::thread_rng();
    thread::sleep(Duration::from_millis(rng.r#gen_range(200..500)));
    
    state.functions_detected = rng.r#gen_range(10..30);
    state.analysis_done = true;
    
    println!("[{}] Functions found: {}", "aa".cyan(), state.functions_detected);
    println!("{}", "Basic analysis done. For more thorough lies, try 'aaa'.".dimmed());
}

/// Extra analysis - aaaa command
pub fn analyze_extra(state: &mut ShellState) {
    analyze_all(state);
    
    println!();
    println!("{}", "╔═══════════════════════════════════════╗".bright_magenta());
    println!("{}", "║     ✨ EXTRA ANALYSIS COMPLETE ✨     ║".bright_magenta());
    println!("{}", "╠═══════════════════════════════════════╣".bright_magenta());
    println!("{}", "║  Additional discoveries:              ║".bright_magenta());
    println!("{}", "║  • Hidden secrets: probably           ║".bright_magenta());
    println!("{}", "║  • Buffer overflows: definitely       ║".bright_magenta());
    println!("{}", "║  • Job security: questionable         ║".bright_magenta());
    println!("{}", "║  • Easter eggs: keep looking          ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════╝".bright_magenta());
}

/// Analyze function - af command
pub fn analyze_function(state: &mut ShellState, args: &[&str]) {
    let addr = if args.is_empty() {
        state.current_address
    } else {
        match crate::cli::parser::parse_address(args[0]) {
            Some(a) => a,
            None => {
                println!("{}", "Invalid address. Can't analyze imaginary locations.".red());
                return;
            }
        }
    };

    println!("[{}] Analyzing function at 0x{:08x}...", "af".cyan(), addr);
    
    let mut rng = rand::thread_rng();
    thread::sleep(Duration::from_millis(rng.r#gen_range(100..300)));
    
    let size = rng.r#gen_range(20..500);
    let blocks = rng.r#gen_range(1..20);
    let locals = rng.r#gen_range(0..10);
    let args_count = rng.r#gen_range(0..6);
    
    println!();
    println!("  {} {} @ 0x{:08x}", "function:".dimmed(), "fcn.detected".bright_cyan(), addr);
    println!("  {} {} bytes", "size:".dimmed(), size);
    println!("  {} {}", "blocks:".dimmed(), blocks);
    println!("  {} {}", "locals:".dimmed(), locals);
    println!("  {} {} (probably)", "args:".dimmed(), args_count);
    println!("  {} {}", "calling convention:".dimmed(), "yolo".bright_yellow());
    println!("  {} {}", "complexity:".dimmed(), 
        match blocks {
            1..=3 => "trivial",
            4..=10 => "reasonable", 
            11..=15 => "concerning",
            _ => "spaghetti",
        }.yellow()
    );

    state.functions_detected += 1;
}
