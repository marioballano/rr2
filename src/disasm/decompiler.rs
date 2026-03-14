//! Decompiler - turning assembly into creative fiction

use colored::*;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::core::state::ShellState;
use crate::analysis::functions;

/// Decompile function - pdc command
pub fn decompile_function(state: &mut ShellState) {
    let addr = state.current_address;
    let func = functions::generate_function_info(addr);
    
    println!();
    println!("{}", "/* ═══════════════════════════════════════════════════════════════ */".dimmed());
    println!("{}", format!("/*  Decompiled by rr2 - Reliability: {}%                      */", 
        rand::thread_rng().r#gen_range(1..30)).dimmed());
    println!("{}", "/*  WARNING: This is creative fiction, not actual source code      */".dimmed());
    println!("{}", "/* ═══════════════════════════════════════════════════════════════ */".dimmed());
    println!();

    println!("{}", generate_decompiled_function(&func));
}

/// Optimized decompiler - pdco command
pub fn decompile_optimized(state: &mut ShellState) {
    let addr = state.current_address;
    let func = functions::generate_function_info(addr);
    
    println!();
    println!("{}", "/* OPTIMIZED DECOMPILATION (even more fictional) */".dimmed());
    println!();

    println!("{}", generate_optimized_decompiled(&func));
}

fn generate_decompiled_function(func: &functions::FakeFunctionInfo) -> String {
    let mut rng = rand::thread_rng();
    let mut output = String::new();
    
    // Function signature
    let return_type = ["int", "void", "char *", "int64_t", "void *", "size_t"]
        .choose(&mut rng).unwrap();
    
    let arg_names = ["argc", "argv", "buf", "size", "ptr", "len", "fd", "flags", "ctx", "data"];
    let arg_types = ["int", "char **", "void *", "size_t", "int", "uint64_t", "const char *"];
    
    let mut args = Vec::new();
    for i in 0..func.args.min(4) {
        let t = arg_types.choose(&mut rng).unwrap();
        let n = arg_names[i];
        args.push(format!("{} {}", t, n));
    }
    
    let args_str = if args.is_empty() {
        "void".to_string()
    } else {
        args.join(", ")
    };
    
    // Function header
    output.push_str(&format!("{}", format!("{} fcn_{}({}) {{", return_type, func.name, args_str).cyan()));
    output.push('\n');
    
    // Local variables
    let local_types = ["int", "char", "void *", "uint64_t", "int32_t", "size_t"];
    let local_names = ["result", "tmp", "ptr", "counter", "buffer", "status", "flags", "offset", "idx"];
    
    for i in 0..func.locals.min(5) {
        let t = local_types.choose(&mut rng).unwrap();
        let n = local_names[i];
        let init = match *t {
            "int" | "int32_t" => rng.r#gen_range(-100..100).to_string(),
            "uint64_t" | "size_t" => format!("0x{:x}", rng.r#gen_range(0..0xFFFF)),
            "void *" | "char *" => "NULL".to_string(),
            "char" => format!("'{}'", (rng.r#gen_range(b'a'..b'z') as char)),
            _ => "0".to_string(),
        };
        output.push_str(&format!("    {} {} = {};", t, n.yellow(), init.green()));
        if rng.r#gen_bool(0.3) {
            output.push_str(&format!(" {}", "// probably wrong".dimmed()));
        }
        output.push('\n');
    }
    
    if func.locals > 0 {
        output.push('\n');
    }
    
    // Function body
    let statements = generate_statements(&mut rng, func.blocks.min(8));
    for stmt in statements {
        output.push_str(&format!("    {}\n", stmt));
    }
    
    // Return
    if *return_type != "void" {
        output.push('\n');
        let ret_val = match *return_type {
            "int" | "int64_t" => rng.r#gen_range(0..3).to_string(),
            "char *" | "void *" => "ptr".to_string(),
            "size_t" => "result".to_string(),
            _ => "0".to_string(),
        };
        let ret_comment = ["// success?", "// lol no", "// optimistically", "// TODO verify", "// trust me"]
            .choose(&mut rng).unwrap();
        output.push_str(&format!("    return {}; {}\n", ret_val.green(), ret_comment.dimmed()));
    }
    
    output.push_str(&"}".cyan().to_string());
    output.push('\n');
    
    output
}

fn generate_statements(rng: &mut impl Rng, count: usize) -> Vec<String> {
    let mut statements = Vec::new();
    
    let statement_templates = [
        // Conditionals
        vec![
            "if (undefined_behavior) {".to_string(),
            "    goto error;".to_string(),
            "}".to_string(),
        ],
        vec![
            "if (ptr == NULL) {".to_string(),
            "    // this should never happen".to_string(),
            "    return -1; // narrator: it happened".to_string(),
            "}".to_string(),
        ],
        vec![
            "if (size > MAX_BUFFER_SIZE) {".to_string(),
            "    // what's a bounds check?".to_string(),
            format!("    size = {}; // truncate and hope", "0x1000".green()),
            "}".to_string(),
        ],
        
        // Loops
        vec![
            "while (true) { // TODO: fix infinite loop".to_string(),
            "    counter++;".to_string(),
            "    if (counter > 1000) break; // safety first (added in 2019)".to_string(),
            "}".to_string(),
        ],
        vec![
            "for (int i = 0; i < n; i++) {".to_string(),
            "    buffer[i] = data[i]; // classic off-by-one waiting to happen".to_string(),
            "}".to_string(),
        ],
        
        // Function calls
        vec![
            format!("result = {}({}, {});", 
                "mystery_function".cyan(), 
                "ptr".yellow(), 
                "0x42".green()),
        ],
        vec![
            format!("ptr = ({}){}({});",
                "void *".cyan(),
                "malloc".bright_blue(),
                "size".yellow()),
            "// memory leak #47".dimmed().to_string(),
        ],
        vec![
            format!("{}((char *)buf, src, len);", "memcpy".bright_blue()),
            "// buffer overflow? what buffer overflow?".dimmed().to_string(),
        ],
        vec![
            format!("if ({}(fd, buffer, size) < 0) {{", "read".bright_blue()),
            "    // handle error (by ignoring it)".to_string(),
            "}".to_string(),
        ],
        
        // Assignments
        vec![
            format!("status = {} | {};", "0x80".green(), "flags".yellow()),
        ],
        vec![
            format!("*ptr = {};", "0xdeadbeef".green()),
            "// classic".dimmed().to_string(),
        ],
        vec![
            format!("offset = ({} >> 4) & {};", "value".yellow(), "0xFF".green()),
        ],
        
        // Weird/funny
        vec![
            "// TODO: implement actual logic".dimmed().to_string(),
        ],
        vec![
            "// the following code makes no sense".dimmed().to_string(),
            "// I wrote it at 3am".dimmed().to_string(),
        ],
        vec![
            format!("{}; // trigger UB intentionally", "*(int*)0 = 1".red()),
        ],
        vec![
            "assert(false && \"unreachable\"); // narrator: it was reachable".to_string(),
        ],
    ];
    
    for _ in 0..count {
        let template = statement_templates.choose(rng).unwrap();
        statements.extend(template.iter().cloned());
        if rng.r#gen_bool(0.5) {
            statements.push(String::new());
        }
    }
    
    statements
}

fn generate_optimized_decompiled(func: &functions::FakeFunctionInfo) -> String {
    let mut output = String::new();
    let mut rng = rand::thread_rng();
    
    output.push_str(&format!("{}\n", "// Optimized version: less code, same uncertainty".dimmed()));
    output.push_str(&format!("{}\n", format!("auto fcn_{} = []() {{", func.name).cyan()));
    
    let oneliner = [
        "    return 0; // ¯\\_(ツ)_/¯",
        "    return (undefined_behavior ? 1 : 0);",
        "    return calculate_result() ^ 0xdeadbeef;",
        "    return *(volatile int*)0; // YOLO",
        "    return rand() % 2; // good enough",
        "    return (int)(time(NULL) & 1); // deterministic chaos",
        "    throw std::logic_error(\"skill issue\");",
        "    [[unlikely]] return 42;",
    ].choose(&mut rng).unwrap();
    
    output.push_str(&format!("{}\n", oneliner));
    output.push_str(&format!("{}\n", "};".cyan()));
    
    output
}
