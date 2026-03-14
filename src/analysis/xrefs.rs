//! Cross-references - the spaghetti map of your binary

use colored::*;
use rand::Rng;

use crate::core::state::ShellState;

/// List all xrefs - ax command
pub fn list_xrefs(state: &mut ShellState) {
    if !state.analysis_done {
        println!("{}", "Run 'aaa' first to detect xrefs. Or don't. We'll improvise.".yellow());
    }

    let mut rng = rand::thread_rng();
    let count = state.xrefs_resolved.max(rng.r#gen_range(20..50));
    let entry = state.binary_info.as_ref()
        .map(|b| b.entry_point)
        .unwrap_or(0x00400000);

    println!("{}", "Cross-references (the tangled web we weave):".bright_cyan());
    println!();

    let xref_types = ["CALL", "JMP", "DATA", "CODE", "STR"];

    for _i in 0..count.min(25) {
        let from_addr = entry + rng.r#gen_range(0..0x5000);
        let to_addr = entry + rng.r#gen_range(0..0x5000);
        let xref_type = xref_types[rng.r#gen_range(0..xref_types.len())];

        let type_color = match xref_type {
            "CALL" => xref_type.green(),
            "JMP" => xref_type.yellow(),
            "DATA" => xref_type.cyan(),
            "CODE" => xref_type.white(),
            "STR" => xref_type.magenta(),
            _ => xref_type.dimmed(),
        };

        println!("  {} 0x{:08x} -> 0x{:08x} {}",
            type_color,
            from_addr,
            to_addr,
            format!("[{}]", fake_xref_context(&mut rng)).dimmed()
        );
    }

    if count > 25 {
        println!("{}", format!("  ... and {} more xrefs (it's xrefs all the way down)", count - 25).dimmed());
    }

    println!();
    println!("{}", "Legend: It's complicated. Everything references everything.".dimmed());
}

/// Xrefs to address - axt command
pub fn xrefs_to(state: &mut ShellState, args: &[&str]) {
    let addr = if args.is_empty() {
        state.current_address
    } else {
        match crate::cli::parser::parse_address(args[0]) {
            Some(a) => a,
            None => {
                println!("{}", "Invalid address. Try a real one.".red());
                return;
            }
        }
    };

    println!("{} 0x{:08x}:", "References TO".bright_cyan(), addr);
    println!();

    let mut rng = rand::thread_rng();
    let count = rng.r#gen_range(1..8);
    let base = state.binary_info.as_ref()
        .map(|b| b.entry_point)
        .unwrap_or(0x00400000);

    for _i in 0..count {
        let from_addr = base + rng.r#gen_range(0..0x5000);
        let xref_type = if rng.r#gen_bool(0.7) { "CALL" } else { "JMP" };
        
        println!("  {} 0x{:08x} {} {}",
            "←".green(),
            from_addr,
            xref_type.yellow(),
            format!("fcn.{:05x}", from_addr % 0x100000).dimmed()
        );
    }

    if count == 0 {
        println!("{}", "  No references found. This code is lonely.".dimmed());
    }

    println!();
    println!("{}", format!("Total: {} xrefs pointing here", count).dimmed());
}

/// Xrefs from address - axf command
pub fn xrefs_from(state: &mut ShellState, args: &[&str]) {
    let addr = if args.is_empty() {
        state.current_address
    } else {
        match crate::cli::parser::parse_address(args[0]) {
            Some(a) => a,
            None => {
                println!("{}", "Invalid address. Numbers go here.".red());
                return;
            }
        }
    };

    println!("{} 0x{:08x}:", "References FROM".bright_cyan(), addr);
    println!();

    let mut rng = rand::thread_rng();
    let count = rng.r#gen_range(1..10);
    let base = state.binary_info.as_ref()
        .map(|b| b.entry_point)
        .unwrap_or(0x00400000);

    for _i in 0..count {
        let to_addr = base + rng.r#gen_range(0..0x5000);
        let xref_type = ["CALL", "JMP", "DATA"][rng.r#gen_range(0..3)];
        
        let target_name = match xref_type {
            "CALL" => format!("fcn.{:05x}", to_addr % 0x100000),
            "DATA" => format!("str.{:05x}", to_addr % 0x100000),
            "JMP" => format!("loc.{:05x}", to_addr % 0x100000),
            _ => "???".to_string(),
        };

        println!("  {} 0x{:08x} {} {}",
            "→".cyan(),
            to_addr,
            xref_type.yellow(),
            target_name.dimmed()
        );
    }

    if count == 0 {
        println!("{}", "  This code references nothing. Very zen.".dimmed());
    }

    println!();
    println!("{}", format!("Total: {} xrefs from here", count).dimmed());
}

fn fake_xref_context(rng: &mut impl Rng) -> &'static str {
    const CONTEXTS: &[&str] = &[
        "in main",
        "in entry0",
        "in fcn.init",
        "via PLT",
        "indirect",
        "direct call",
        "computed",
        "switch case",
        "error handler",
        "cleanup routine",
        "initialization",
        "callback",
        "event handler",
        "signal handler",
        "vtable entry",
    ];
    CONTEXTS[rng.r#gen_range(0..CONTEXTS.len())]
}
