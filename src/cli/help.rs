//! Help - assistance for the lost souls

use colored::*;

pub fn show_help(args: &[&str]) {
    if args.is_empty() {
        show_main_help();
    } else {
        match args[0] {
            "a" | "analysis" => show_analysis_help(),
            "p" | "print" => show_print_help(),
            "i" | "info" => show_info_help(),
            "s" | "seek" => show_seek_help(),
            "V" | "visual" => show_visual_help(),
            "e" | "config" => show_config_help(),
            "d" | "debug" => show_debug_help(),
            _ => show_main_help(),
        }
    }
}

pub fn show_main_help() {
    let help = r#"
╔══════════════════════════════════════════════════════════════════════════════╗
║                          rr2 COMMAND REFERENCE                               ║
║                    "It's like radare2, but worse!"                           ║
╠══════════════════════════════════════════════════════════════════════════════╣
║  ANALYSIS (a)                                                                ║
║    aaa          Analyze all (allegedly)         af           Analyze function║
║    afl          List functions                  ax           List xrefs      ║
║                                                                              ║
║  PRINT (p)                                                                   ║
║    pd [n]       Print n disassembly lines       pdf          Print function  ║
║    pdc          Decompile function              px [n]       Print hex dump  ║
║    ps           Print string                                                 ║
║                                                                              ║
║  INFO (i)                                                                    ║
║    i            Show binary info                ie           Show entry point║
║    ii           Show imports                    is           Show symbols    ║
║    iS           Show sections                   iz           Show strings    ║
║                                                                              ║
║  SEEK (s)                                                                    ║
║    s [addr]     Seek to address                 s+/s-        Seek relative   ║
║    sf           Seek to function                                             ║
║                                                                              ║
║  VISUAL (V)                                                                  ║
║    V            Visual mode (lol)               VV           ASCII graph mode║
║                                                                              ║
║  CONFIG (e)                                                                  ║
║    e            List config                     e key=val    Set config value║
║                                                                              ║
║  OTHER                                                                       ║
║    q            Quit                            ?            This help       ║
║    ??           Extended help                   ?!           About rr2       ║
║    fortune      Random wisdom                                                ║
║                                                                              ║
╠══════════════════════════════════════════════════════════════════════════════╣
║  Use '?' after any command for detailed help (e.g., 'a?', 'p?')              ║
╚══════════════════════════════════════════════════════════════════════════════╝
"#;
    println!("{}", help.cyan());
}

pub fn show_extended_help() {
    println!("{}", r#"
╔══════════════════════════════════════════════════════════════════════════════╗
║                          EXTENDED HELP (for the brave)                       ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  COMMAND MODIFIERS:                                                          ║
║    cmd j        JSON output (it's still fake, just formatted differently)    ║
║    cmd *        Apply to all (whatever "all" means)                          ║
║    cmd @addr    Execute at address (we'll pretend to go there)               ║
║                                                                              ║
║  SPECIAL COMMANDS:                                                           ║
║    !cmd         Shell escape (not really)                                    ║
║    #comment     Comments (we read them... sometimes)                         ║
║    .script      Run script (imagination required)                            ║
║                                                                              ║
║  NAVIGATION:                                                                 ║
║    Enter        Repeat last command (or do nothing, depending on our mood)   ║
║    Ctrl+C       Interrupt (we'll guilt-trip you)                             ║
║    Ctrl+D       EOF (dramatic exit)                                          ║
║                                                                              ║
║  PRO TIPS:                                                                   ║
║    - Type 'fortune' when sad                                                 ║
║    - Try typing random hex values (secrets await)                            ║
║    - The decompiler is a work of fiction                                     ║
║    - None of this is real                                                    ║
║    - Have you tried IDA? (Just kidding, this is better*)                     ║
║      *Citation needed                                                        ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
"#.bright_yellow());
}

pub fn show_about() {
    println!("{}", r#"
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║     ██████╗ ██████╗ ██████╗                                  ║
    ║     ██╔══██╗██╔══██╗╚════██╗                                 ║
    ║     ██████╔╝██████╔╝ █████╔╝                                 ║
    ║     ██╔══██╗██╔══██╗██╔═══╝                                  ║
    ║     ██║  ██║██║  ██║███████╗                                 ║
    ║     ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝                                 ║
    ║                                                              ║
    ║     radare rust 2                                            ║
    ║     "where undefined behavior is a feature"                  ║
    ║                                                              ║
    ║     Version: 0.1.337                                         ║
    ║     License: WTFPL                                           ║
    ║     Authors: The Segfault Collective                         ║
    ║                                                              ║
    ║     This tool is a parody for entertainment purposes.        ║
    ║     No binaries were harmed in the making of this tool.      ║
    ║     (They were already broken when they got here.)           ║
    ║                                                              ║
    ║     In loving memory of all the stack frames                 ║
    ║     that were smashed in the name of progress.               ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝
"#.bright_magenta());
}

pub fn show_analysis_help() {
    println!("{}", r#"
ANALYSIS COMMANDS (a) - "We're totally analyzing stuff"

  aaa           Analyze all automatically (quotes intentional)
  aaaa          Analyze EVEN MORE (for the paranoid)
  aa            Analyze basic (for the lazy)
  
  af            Analyze function at current address
  af addr       Analyze function at specific address
  afl           List all detected functions
  afll          List functions (long format, more lies)
  afn name      Rename function (cosmetic surgery for binaries)
  
  ax            List all xrefs (cross-references)
  axt addr      List xrefs TO address
  axf addr      List xrefs FROM address
  
Pro Tip: The more 'a's you type, the more analysis happens.
         (This is false, but it feels true.)
"#.cyan());
}

pub fn show_print_help() {
    println!("{}", r#"
PRINT COMMANDS (p) - "Making bytes look pretty"

  pd [n]        Print n disassembly instructions (default: 10)
  pdf           Print disassembly of current function
  pdc           Print decompiled code (creative fiction)
  pdco          Print optimized decompiler output (less verbose fiction)
  
  px [n]        Print n bytes as hex dump (default: 64)
  pxw [n]       Print hex dump as words (4 bytes)
  pxq [n]       Print hex dump as qwords (8 bytes)
  pxr [n]       Print hex dump with references
  
  ps            Print string at current address
  psz           Print zero-terminated string
  psp           Print Pascal-style string

Note: The decompiler is works of fiction. Any resemblance to 
      actual source code is purely coincidental.
"#.cyan());
}

pub fn show_info_help() {
    println!("{}", r#"
INFO COMMANDS (i) - "Learning about your binary"

  i             Show general binary information
  ie            Show entry point(s)
  ih            Show headers
  ii            Show imports
  iE            Show exports
  is            Show symbols
  iS            Show sections
  il            Show linked libraries
  iz            Show strings in data sections
  izz           Show ALL strings (chaos mode)

Most of this is real-ish. We actually use goblin 
for parsing. The rest... creative interpretation.
"#.cyan());
}

pub fn show_seek_help() {
    println!("{}", r#"
SEEK COMMANDS (s) - "Teleportation for bytes"

  s             Print current address
  s addr        Seek to address
  s+ [n]        Seek forward n bytes (default: 1)
  s- [n]        Seek backward n bytes (default: 1)
  sf name       Seek to function by name
  sr            Seek to return address (best guess)
  
Special addresses:
  s main        Seek to main function
  s entry       Seek to entry point
  s 0xdeadbeef  Seek to the forbidden address

Warning: Seeking to invalid addresses may cause
         existential dread. We are not responsible.
"#.cyan());
}

pub fn show_visual_help() {
    println!("{}", r#"
VISUAL COMMANDS (V) - "ASCII art is peak UX"

  V             Enter visual mode (experience required)
  VV            ASCII graph view (prepare for art)
  v             Mini visual mode (for small screens)

Visual mode is a state of mind more than a feature.
Think of it as meditation with hexadecimal.

Keys in visual mode (theoretically):
  hjkl/arrows   Navigate
  q             Exit (back to sanity)
  ?             Help (more of this)
  p/P           Change print mode
  c             Toggle cursor

Disclaimer: Visual mode may or may not work.
"#.cyan());
}

pub fn show_config_help() {
    println!("{}", r#"
CONFIG COMMANDS (e) - "Settings that may or may not do things"

  e             List all config values
  e key         Get specific config value
  e key=value   Set config value

Popular settings:
  asm.arch      Architecture (we ignore this)
  asm.bits      Bits (also ignored)
  asm.syntax    intel/att (intel is correct, fight me)
  asm.comments  Show comments (always true here)
  asm.sarcasm   Sarcasm level (default: maximum)
  
  scr.color     Enable colors (yes please)
  cfg.fortunes  Show fortunes (why would you disable this?)
  
  dbg.backend   Debugger backend (options: prayer, hope, caffeine)
  user.sanity   Your mental state (default: questionable)
"#.cyan());
}

pub fn show_debug_help() {
    println!("{}", r#"
DEBUG COMMANDS (d) - "Pretending to debug"

  db [addr]     Set breakpoint
  dc            Continue execution
  dr            Show registers
  ds            Step one instruction
  dso           Step over
  dbt           Show backtrace

IMPORTANT: This is not a debugger.
We just print things that look debugger-ish.
For actual debugging, use gdb, lldb, or tears.
"#.cyan());
}

pub fn show_file_help() {
    println!("{}", r#"
FILE COMMANDS (o) - "Opening things"

  o             Show open files
  o file        Open file (not really, we keep the current one)
  oo            Reopen in write mode
  oo+           Reopen in WRITE mode (more eager)

We're not in the business of opening multiple files.
That's complexity. We prefer simplicity.
(We're also lazy.)
"#.cyan());
}

pub fn show_write_help() {
    println!("{}", r#"
WRITE COMMANDS (w) - "Danger zone"

  w string      Write string (if -w flag was used)
  wx hex        Write hex bytes
  wv value      Write value
  
Note: Write mode is disabled by default because:
  1. We don't trust you
  2. We don't trust ourselves
  3. It's safer this way

Use -w flag to enable writes. Consequences are your own.
"#.cyan());
}

pub fn show_search_help() {
    println!("{}", r#"
SEARCH COMMANDS (/) - "Finding needles in haystacks"

  /             Search for... something
  /x hex        Search for hex pattern
  /s string     Search for string
  // comment    Search for meaning in life (not found)

Search functionality is optimized for finding nothing.
If you actually need to search, try grep.
"#.cyan());
}

pub fn show_project_help() {
    println!("{}", r#"
PROJECT COMMANDS (P) - "Persistence is overrated"

  Ps            Save project (to /dev/null)
  Po            Open project (from your imagination)
  Pi            Project info
  
Projects are a commitment we're not ready for.
Live in the moment. Analyze in the now.
"#.cyan());
}
