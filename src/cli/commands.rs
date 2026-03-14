//! Command handler - routing chaos since 2024

use colored::*;
use crate::core::state::ShellState;
use crate::analysis::{fake_anal, functions, xrefs};
use crate::disasm::{fake_disasm, decompiler};
use crate::print::{hex, strings};
use crate::binary::info;
use crate::visual::{mode, graphs};
use crate::humor::{easter_eggs, insults};
use crate::cli::help;

pub struct CommandHandler;

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler
    }

    pub fn handle(&self, input: &str, state: &mut ShellState) {
        let input = input.trim();
        
        if input.is_empty() {
            println!("{}", insults::empty_input());
            return;
        }

        // Check for Easter eggs first
        if let Some(egg) = easter_eggs::check_easter_egg(input) {
            println!("{}", egg);
            return;
        }

        // Parse command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.first().unwrap_or(&"");
        let args: Vec<&str> = parts.iter().skip(1).cloned().collect();

        match *cmd {
            // ==================== Help Commands ====================
            "?" | "help" => help::show_help(&args),
            "??" => help::show_extended_help(),
            "?!" => help::show_about(),

            // ==================== Analysis Commands ====================
            "aaa" => fake_anal::analyze_all(state),
            "aa" => fake_anal::analyze_basic(state),
            "aaaa" => {
                println!("{}", "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".bright_yellow());
                println!("{}", "Okay okay, we get it. Running EXTRA analysis...".dimmed());
                fake_anal::analyze_extra(state);
            }
            "af" => fake_anal::analyze_function(state, &args),
            "afl" => functions::list_functions(state),
            "afll" => functions::list_functions_long(state),
            "afn" => functions::rename_function(state, &args),
            "ax" => xrefs::list_xrefs(state),
            "axt" => xrefs::xrefs_to(state, &args),
            "axf" => xrefs::xrefs_from(state, &args),
            
            // Analysis help
            "a?" | "af?" | "ax?" => help::show_analysis_help(),

            // ==================== Print Commands ====================
            "pd" => fake_disasm::print_disassembly(state, &args),
            "pdf" => fake_disasm::print_function_disassembly(state),
            "pdc" => decompiler::decompile_function(state),
            "pdco" => decompiler::decompile_optimized(state),
            "px" => hex::print_hex(state, &args),
            "pxw" => hex::print_hex_words(state, &args),
            "pxq" => hex::print_hex_qwords(state, &args),
            "pxr" => hex::print_hex_refs(state, &args),
            "ps" => strings::print_string(state, &args),
            "psz" => strings::print_string_zero(state, &args),
            "psp" => strings::print_string_pascal(state, &args),

            // Print help
            "p?" | "pd?" | "px?" | "ps?" => help::show_print_help(),

            // ==================== Info Commands ====================
            "i" => info::show_info(state),
            "ie" => info::show_entry(state),
            "ii" => info::show_imports(state),
            "iE" => info::show_exports(state),
            "is" => info::show_symbols(state),
            "iS" => info::show_sections(state),
            "ih" => info::show_headers(state),
            "il" => info::show_libraries(state),
            "iz" => strings::list_strings(state),
            "izz" => strings::list_all_strings(state),

            // Info help
            "i?" => help::show_info_help(),

            // ==================== Seek Commands ====================
            "s" => {
                if args.is_empty() {
                    println!("0x{:08x}", state.current_address);
                } else {
                    seek_to(state, args[0]);
                }
            }
            "s+" => seek_relative(state, &args, true),
            "s-" => seek_relative(state, &args, false),
            "sf" => seek_to_function(state, &args),
            "sr" => {
                // Seek to return address (fake)
                println!("{}", "Seeking to return address... if only we knew where that was.".dimmed());
            }

            // Seek help
            "s?" => help::show_seek_help(),

            // ==================== Visual Commands ====================
            "V" => mode::enter_visual_mode(state),
            "VV" => graphs::show_graph(state),
            "v" => mode::mini_visual(state),

            // Visual help
            "V?" => help::show_visual_help(),

            // ==================== Config Commands ====================
            "e" => {
                if args.is_empty() {
                    show_config(state);
                } else if args[0].contains('=') {
                    set_config(state, args[0]);
                } else {
                    get_config(state, args[0]);
                }
            }
            "e?" => help::show_config_help(),

            // ==================== File Commands ====================
            "o" => {
                if args.is_empty() {
                    show_open_files(state);
                } else {
                    println!("{}", format!("Opening '{}' (just kidding, we're keeping the current one)", args[0]).dimmed());
                }
            }
            "oo" => {
                println!("{}", "Reopening file in write mode... (not really)".yellow());
            }
            "oo+" => {
                println!("{}", "Reopening file in VERY write mode... (still not really)".yellow());
            }
            "o?" => help::show_file_help(),

            // ==================== Write Commands ====================
            "w" | "wx" => {
                if state.write_mode {
                    println!("{}", "Write command received. Bytes written: 0 (safety first!)".yellow());
                } else {
                    println!("{}", "File not opened in write mode. Use -w flag (but don't, really)".red());
                }
            }
            "w?" => help::show_write_help(),

            // ==================== Search Commands ====================
            "/" => {
                println!("{}", "Searching for existential meaning... not found.".dimmed());
            }
            "/x" => {
                println!("{}", "Hex search: Results may vary (they will vary to 'nothing')".dimmed());
            }
            "/?" => help::show_search_help(),

            // ==================== Debug Commands ====================
            "db" => println!("{}", "Breakpoint set! (It's imaginary, like your job security)".cyan()),
            "dc" => println!("{}", "Continuing execution... to nowhere, because this isn't a debugger.".cyan()),
            "dr" => println!("{}", "Registers: All of them are probably wrong.".cyan()),
            "ds" => println!("{}", "Stepping... into the void.".cyan()),
            "d?" => help::show_debug_help(),

            // ==================== Project Commands ====================
            "P" => println!("{}", "Project commands: We don't save state, we live in the moment.".dimmed()),
            "Po" => println!("{}", "Loading project... JK, there are no projects.".dimmed()),
            "Ps" => println!("{}", "Project saved to /dev/null successfully!".green()),
            "P?" => help::show_project_help(),

            // ==================== Misc Commands ====================
            "!" => {
                // Shell escape
                if args.is_empty() {
                    println!("{}", "Usage: !<command> - but we won't actually run it".dimmed());
                } else {
                    println!("{}", format!("Shell command '{}' executed successfully! (not really)", args.join(" ")).dimmed());
                }
            }
            "." => println!("{}", "Interpret script: Scripts are just commands with commitment issues.".dimmed()),
            "#" => println!("{}", "This is a comment. Nobody reads comments. Except you, apparently.".dimmed()),
            "(" => println!("{}", "Macro definition: For when you want to automate your confusion.".dimmed()),
            
            "fortune" | "fo" => {
                println!(" -- {}", crate::humor::fortunes::get_random().green());
            }

            // ==================== Quit Commands ====================
            "q" | "quit" | "exit" => {
                println!("{}", "Goodbye! May your binaries be bug-free (lol).".yellow());
                std::process::exit(0);
            }
            "q!" => {
                println!("{}", "Force quitting! No questions asked!".red());
                std::process::exit(0);
            }
            "Q" => {
                println!("{}", "DRAMATICALLY QUITTING!".bright_red().bold());
                std::process::exit(0);
            }

            // ==================== Unknown Command ====================
            _ => {
                // Check for similarity to known commands
                if let Some(suggestion) = insults::suggest_command(cmd) {
                    println!("{}", suggestion);
                } else {
                    println!("{}", insults::unknown_command());
                }
            }
        }
    }
}

impl Default for CommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

fn seek_to(state: &mut ShellState, addr_str: &str) {
    // Handle special addresses
    match addr_str.to_lowercase().as_str() {
        "main" | "sym.main" | "entry" | "entry0" => {
            let entry = state.binary_info.as_ref()
                .map(|b| b.entry_point)
                .unwrap_or(0x00400000);
            state.seek(entry);
            println!("{} 0x{:08x}", "Seeking to".dimmed(), entry);
            return;
        }
        _ => {}
    }
    
    // Try to parse as hex or decimal
    let addr = if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
        u64::from_str_radix(&addr_str[2..], 16)
    } else if addr_str.chars().all(|c| c.is_ascii_hexdigit()) {
        u64::from_str_radix(addr_str, 16)
    } else {
        addr_str.parse::<u64>()
    };

    match addr {
        Ok(a) => {
            state.seek(a);
            // Check for address Easter eggs
            if let Some(egg) = easter_eggs::check_address_easter_egg(a) {
                println!("{}", egg.yellow());
            }
            println!("{} 0x{:08x}", "Seeking to".dimmed(), a);
        }
        Err(_) => {
            println!("{}", insults::invalid_address());
        }
    }
}

fn seek_relative(state: &mut ShellState, args: &[&str], forward: bool) {
    let offset: i64 = if args.is_empty() {
        1
    } else {
        args[0].parse().unwrap_or(1)
    };
    
    let actual_offset = if forward { offset } else { -offset };
    state.seek_relative(actual_offset);
    println!("{} 0x{:08x}", "Now at".dimmed(), state.current_address);
}

fn seek_to_function(_state: &mut ShellState, args: &[&str]) {
    if args.is_empty() {
        println!("{}", "Usage: sf <function_name>".dimmed());
    } else {
        println!("{}", format!("Seeking to function '{}' (if only we knew where it was)", args[0]).dimmed());
    }
}

fn show_config(state: &ShellState) {
    println!("{}", "Configuration (some of it might even be real):".bright_cyan());
    for (key, value) in state.config.list() {
        println!("  {} = {}", key.cyan(), value.yellow());
    }
}

fn get_config(state: &ShellState, key: &str) {
    if let Some(value) = state.config.get(key) {
        println!("{} = {}", key.cyan(), value.yellow());
    } else {
        println!("{}", format!("Config '{}' not found. We made it up anyway.", key).red());
    }
}

fn set_config(state: &mut ShellState, kv: &str) {
    let parts: Vec<&str> = kv.splitn(2, '=').collect();
    if parts.len() == 2 {
        state.config.set(parts[0], parts[1]);
        println!("{}", format!("Set {} = {} (probably won't change anything)", parts[0], parts[1]).green());
    } else {
        println!("{}", "Invalid syntax. Use: e key=value".red());
    }
}

fn show_open_files(state: &ShellState) {
    println!("{}", "Open files:".bright_cyan());
    if state.has_file() {
        println!("  {} {}", "*".green(), state.file_name());
    } else {
        println!("  {} malloc://512 {}", "*".green(), "(the void)".dimmed());
    }
}
