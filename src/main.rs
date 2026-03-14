//! rr2 - radare rust 2
//! "where undefined behavior is a feature"

use clap::Parser;
use colored::*;
use std::path::PathBuf;

mod core;
mod cli;
mod analysis;
mod disasm;
mod binary;
mod print;
mod humor;
mod visual;

use crate::core::shell::Shell;
use crate::humor::fortunes;

#[derive(Parser, Debug)]
#[command(name = "rr2")]
#[command(author = "The Segfault Collective")]
#[command(version = "0.1.337")]
#[command(about = "radare rust 2 - where undefined behavior is a feature", long_about = None)]
struct Args {
    /// Binary file to analyze (optional, we'll pretend either way)
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    /// Quiet mode (suppress the banner, you monster)
    #[arg(short, long)]
    quiet: bool,

    /// Execute command and exit
    #[arg(short, long, value_name = "CMD")]
    cmd: Option<String>,

    /// Write mode (we promise not to break anything... much)
    #[arg(short, long)]
    write: bool,

    /// Debug mode (enables extra sarcasm)
    #[arg(short, long)]
    debug: bool,

    /// Architecture (ignored, we only speak x86_64)
    #[arg(short, long, default_value = "x86")]
    arch: String,

    /// Bits (16, 32, 64 - we'll pretend to care)
    #[arg(short, long, default_value = "64")]
    bits: u8,
}

fn print_banner() {
    let banner = r#"
                 _____ 
 _ __ _ __ ___  |___ / 
| '__| '__/ _ \   |_ \ 
| |  | | |  __/  ___) |
|_|  |_|  \___| |____/ 
                       "#;
    
    println!("{}", banner.bright_red().bold());
    println!("  {} - {}", 
        "rr2 v0.1.337".bright_cyan(),
        "radare rust 2".dimmed());
    println!("  {}\n", "\"where undefined behavior is a feature\"".yellow().italic());
    
    // Print a random fortune
    println!(" -- {}", fortunes::get_random().green());
    println!();
}

fn main() {
    let args = Args::parse();

    if !args.quiet {
        print_banner();
    }

    // Handle -c (execute command and exit)
    if let Some(cmd) = &args.cmd {
        let mut shell = Shell::new(args.file.clone(), args.write, args.debug);
        shell.execute_command(cmd);
        return;
    }

    // Start interactive shell
    let mut shell = Shell::new(args.file, args.write, args.debug);
    
    if let Err(e) = shell.run() {
        eprintln!("{}: {}", "Fatal error".red().bold(), e);
        eprintln!("{}", "But let's be honest, you probably deserved it.".dimmed());
        std::process::exit(1);
    }
}
