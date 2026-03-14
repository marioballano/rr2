//! Functions - generating fake function lists like it's 2008

use colored::*;
use rand::Rng;

use crate::core::state::ShellState;

/// Fake function names that sound realistic
const FUNCTION_NAMES: &[&str] = &[
    "main",
    "__libc_start_main",
    "__libc_csu_init",
    "__libc_csu_fini",
    "_init",
    "_fini",
    "_start",
    "deregister_tm_clones",
    "register_tm_clones",
    "__do_global_dtors_aux",
    "frame_dummy",
    "check_password",
    "validate_input",
    "process_data",
    "encrypt_buffer",
    "decrypt_buffer",
    "handle_connection",
    "parse_config",
    "init_globals",
    "cleanup",
    "debug_print",
    "error_handler",
    "malloc_wrapper",
    "free_wrapper",
    "calculate_checksum",
    "verify_signature",
    "load_library",
    "get_user_input",
    "format_output",
    "send_packet",
    "recv_packet",
    "authenticate",
    "authorize",
    "log_event",
    "panic_handler",
    "stack_protector_fail",
    "definitely_not_backdoor",
    "totally_secure_function",
    "TODO_fix_this_later",
    "legacy_code_dont_touch",
    "wtf_does_this_do",
    "written_by_intern",
    "do_the_thing",
    "handle_stuff",
    "process_things",
    "mysterious_function",
    "obfuscated_logic",
    "spaghetti_code",
    "needs_refactoring",
    "probably_vulnerable",
    "buffer_overflow_here",
    "use_after_free_maybe",
    "race_condition_central",
];

/// Fake library function names (imports)
const IMPORT_NAMES: &[&str] = &[
    "printf", "puts", "scanf", "gets", "fgets",
    "malloc", "calloc", "realloc", "free",
    "memcpy", "memset", "memmove", "memcmp",
    "strcpy", "strncpy", "strcat", "strcmp", "strlen",
    "open", "close", "read", "write", "lseek",
    "socket", "connect", "bind", "listen", "accept",
    "fork", "exec", "wait", "exit", "_exit",
    "getenv", "setenv", "system", "popen",
    "fopen", "fclose", "fread", "fwrite", "fseek",
    "pthread_create", "pthread_join", "pthread_mutex_lock",
    "__stack_chk_fail", "__assert_fail",
];

/// List functions - afl command
pub fn list_functions(state: &mut ShellState) {
    let count = if state.analysis_done {
        state.functions_detected
    } else {
        println!("{}", "Warning: Analysis not run. Results will be extra fictional.".yellow());
        println!("{}", "Consider running 'aaa' first. Or don't. We'll make stuff up anyway.".dimmed());
        println!();
        15 // Default fake count
    };

    println!("{}", "Functions detected:".bright_cyan());
    println!("{:<18} {:>4} {:>6} {}", "addr", "bbs", "size", "name");
    println!("{}", "─".repeat(60));

    let mut rng = rand::thread_rng();
    let entry = state.binary_info.as_ref()
        .map(|b| b.entry_point)
        .unwrap_or(0x00400000);

    // Generate function list
    for i in 0..count.min(30) {
        let addr = entry + (i as u64 * rng.r#gen_range(0x20..0x200));
        let blocks = rng.r#gen_range(1..20);
        let size = blocks as usize * rng.r#gen_range(10..50);
        
        let name = if i == 0 {
            "entry0".to_string()
        } else if i == 1 {
            "sym.main".to_string()
        } else if i < 5 {
            format!("sym.{}", FUNCTION_NAMES[rng.r#gen_range(0..10)])
        } else if rng.r#gen_bool(0.3) {
            format!("sym.imp.{}", IMPORT_NAMES[rng.r#gen_range(0..IMPORT_NAMES.len())])
        } else {
            let name_idx = rng.r#gen_range(10..FUNCTION_NAMES.len());
            format!("fcn.{}", FUNCTION_NAMES[name_idx])
        };

        let addr_str = format!("0x{:08x}", addr);
        let color_name = if name.contains("imp.") {
            name.yellow()
        } else if name.contains("main") || name.contains("entry") {
            name.bright_green()
        } else if name.contains("backdoor") || name.contains("vulnerable") {
            name.red()
        } else {
            name.white()
        };

        println!("{} {:>4} {:>6} {}", 
            addr_str.cyan(),
            blocks,
            size,
            color_name
        );
    }

    if count > 30 {
        println!("{}", format!("... and {} more functions (too lazy to show)", count - 30).dimmed());
    }

    println!();
    println!("{}", format!("Total: {} functions (results may be entirely fabricated)", count).dimmed());
}

/// List functions long format - afll command
pub fn list_functions_long(state: &mut ShellState) {
    list_functions(state);
    
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║           EXTENDED FUNCTION INFORMATION                  ║".cyan());
    println!("{}", "╠══════════════════════════════════════════════════════════╣".cyan());
    println!("{}", "║  Cyclomatic complexity:    ████████░░  HIGH              ║".cyan());
    println!("{}", "║  Code coverage:            ██████░░░░  UNKNOWN           ║".cyan());
    println!("{}", "║  Documentation quality:    ░░░░░░░░░░  NONEXISTENT       ║".cyan());
    println!("{}", "║  Likelihood of bugs:       ██████████  YES               ║".cyan());
    println!("{}", "║  Test coverage:            █░░░░░░░░░  HOPEFUL           ║".cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".cyan());
}

/// Rename function - afn command  
pub fn rename_function(_state: &mut ShellState, args: &[&str]) {
    if args.is_empty() {
        println!("{}", "Usage: afn <new_name> [@addr]".dimmed());
        println!("{}", "Example: afn definitely_not_malware @0x00401000".dimmed());
        return;
    }

    let name = args[0];
    let addr = if args.len() > 1 && args[1].starts_with('@') {
        &args[1][1..]
    } else {
        "current"
    };

    println!("{} Function renamed to '{}' at {}", 
        "✓".green(),
        name.bright_cyan(),
        addr
    );
    println!("{}", "(This is purely cosmetic. The binary doesn't care.)".dimmed());

    // Easter eggs for funny names
    if name.to_lowercase().contains("malware") {
        println!("{}", "🚨 Suspicious function name detected. NSA has been notified. 🚨".red());
    } else if name.to_lowercase().contains("backdoor") {
        println!("{}", "Nice try naming it 'backdoor'. Very subtle.".yellow());
    } else if name == "main" {
        println!("{}", "Wow, how original. 'main'. Never seen that before.".dimmed());
    }
}

/// Generate a fake function info block
pub fn generate_function_info(addr: u64) -> FakeFunctionInfo {
    let mut rng = rand::thread_rng();
    
    FakeFunctionInfo {
        address: addr,
        size: rng.r#gen_range(20..500),
        blocks: rng.r#gen_range(1..20),
        name: FUNCTION_NAMES[rng.r#gen_range(0..FUNCTION_NAMES.len())].to_string(),
        args: rng.r#gen_range(0..6),
        locals: rng.r#gen_range(0..10),
        stack_size: rng.r#gen_range(0..256) * 8,
    }
}

#[allow(dead_code)]
pub struct FakeFunctionInfo {
    pub address: u64,
    pub size: usize,
    pub blocks: usize,
    pub name: String,
    pub args: usize,
    pub locals: usize,
    pub stack_size: usize,
}
