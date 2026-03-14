//! Insults - for when users make questionable choices

use rand::seq::SliceRandom;
use colored::*;

const UNKNOWN_COMMAND_RESPONSES: &[&str] = &[
    "Command not found. Have you tried turning it off and never on again?",
    "I don't know that command. Did you mean 'quit'? Please mean 'quit'.",
    "Unknown command. Try 'help' (it won't help, but it exists).",
    "Command not recognized. Are you sure you're not thinking of gdb?",
    "That's not a thing. But nice try!",
    "Error 404: Command not found. Sense of humor also not found.",
    "I don't speak that language. I barely speak assembly.",
    "Invalid command. Have you considered reading the documentation? (There isn't any)",
    "Unknown command. The oracle is confused.",
    "Nope. Try again. (Or don't. I'm not your supervisor.)",
    "Command rejected. The binary gods are displeased.",
    "That command doesn't exist in this dojo.",
    "Unknown command. Did a cat walk on your keyboard?",
    "I'm sorry, Dave. I'm afraid I can't do that.",
    "This isn't the command you're looking for. *waves hand*",
    "Syntax error: brain.exe has stopped working",
    "Command not found in $PATH or in my heart.",
    "Error: Command not found. But you know what I did find? Disappointment.",
    "Unknown command. Perhaps you meant something entirely different?",
    "That's not valid. Neither are your life choices, apparently.",
];

const EMPTY_INPUT_RESPONSES: &[&str] = &[
    "The void stares back.",
    "...",
    "Nothing + Nothing = Still Nothing",
    "*crickets*",
    "You pressed enter with no command. Bold strategy.",
    "Eloquent silence.",
    "The empty set of commands.",
    "null.execute()",
    "void main(void) { return; }",
    "The command was the friends we made along the way. Which is none.",
];

const TYPO_RESPONSES: &[&str] = &[
    "Close, but no cigar. Try again.",
    "Almost! Did butter-fingers strike again?",
    "That's not quite right. Coffee break?",
    "Typo detected. No judgment. Okay, maybe a little judgment.",
    "You were so close! And yet so far.",
];

#[allow(dead_code)]
const FILE_NOT_FOUND_RESPONSES: &[&str] = &[
    "File not found. It's probably hiding from you.",
    "Can't find that file. Did you check under the couch?",
    "File doesn't exist. Neither does my will to continue.",
    "404: File not found. It ran away when it saw your code.",
    "The file has left the building.",
    "File not found. It's in a better place now. (Not your hard drive)",
    "Can't open file. Maybe try being nicer to it?",
];

const INVALID_ADDRESS_RESPONSES: &[&str] = &[
    "Invalid address. This isn't Google Maps.",
    "You can't seek there. Trust me, I've tried.",
    "That address doesn't exist in this reality.",
    "Address out of bounds. Much like your expectations.",
    "Invalid address. Even quantum computing can't find that.",
    "404: Address not found. Have you tried turning the binary off and on?",
    "You can't go there. There be dragons.",
    "Invalid seek. The void says no.",
];

const NO_FILE_LOADED_RESPONSES: &[&str] = &[
    "No file loaded. We're analyzing the void right now.",
    "No binary? No problem! We're just making stuff up anyway.",
    "You haven't loaded a file. What do you expect me to analyze, your hopes and dreams?",
    "No file loaded. Currently analyzing: existential dread",
    "File status: nonexistent, much like your planning skills.",
    "Load a file first. Or don't. We can pretend either way.",
    "No file loaded. Reading from /dev/null instead.",
];

const ANALYSIS_WARNINGS: &[&str] = &[
    "Warning: Analysis may be slightly accurate. Apologies in advance.",
    "Warning: This analysis is best-effort (low effort, actually).",
    "Warning: Side effects may include confusion, laughter, and questioning your career choices.",
    "Warning: Results may vary. By which we mean: they're definitely wrong.",
    "Warning: This tool is for entertainment purposes only. So is your debugging.",
];

pub fn unknown_command() -> String {
    let mut rng = rand::thread_rng();
    UNKNOWN_COMMAND_RESPONSES.choose(&mut rng)
        .unwrap_or(&"Unknown command.")
        .red()
        .to_string()
}

pub fn empty_input() -> String {
    let mut rng = rand::thread_rng();
    EMPTY_INPUT_RESPONSES.choose(&mut rng)
        .unwrap_or(&"...")
        .dimmed()
        .to_string()
}

pub fn typo_hint() -> String {
    let mut rng = rand::thread_rng();
    TYPO_RESPONSES.choose(&mut rng)
        .unwrap_or(&"Typo?")
        .yellow()
        .to_string()
}

#[allow(dead_code)]
pub fn file_not_found() -> String {
    let mut rng = rand::thread_rng();
    FILE_NOT_FOUND_RESPONSES.choose(&mut rng)
        .unwrap_or(&"File not found.")
        .red()
        .to_string()
}

pub fn invalid_address() -> String {
    let mut rng = rand::thread_rng();
    INVALID_ADDRESS_RESPONSES.choose(&mut rng)
        .unwrap_or(&"Invalid address.")
        .red()
        .to_string()
}

pub fn no_file_loaded() -> String {
    let mut rng = rand::thread_rng();
    NO_FILE_LOADED_RESPONSES.choose(&mut rng)
        .unwrap_or(&"No file loaded.")
        .yellow()
        .to_string()
}

pub fn analysis_warning() -> String {
    let mut rng = rand::thread_rng();
    ANALYSIS_WARNINGS.choose(&mut rng)
        .unwrap_or(&"Warning: Results may vary.")
        .yellow()
        .italic()
        .to_string()
}

// Suggest similar commands
pub fn suggest_command(input: &str) -> Option<String> {
    let commands = [
        ("aa", "aaa"), ("aflf", "afl"), ("pf", "pdf"), ("xp", "px"),
        ("qut", "quit"), ("exti", "exit"), ("hep", "help"), ("hlp", "?"),
        ("afl list", "afl"), ("disas", "pd"), ("dissassemble", "pd"),
        ("decompile", "pdc"), ("hex", "px"), ("string", "ps"),
        ("info", "i"), ("seek", "s"), ("goto", "s"),
    ];
    
    for (typo, correct) in commands.iter() {
        if input.to_lowercase().contains(typo) {
            return Some(format!("Did you mean '{}'? {}", correct.cyan(), typo_hint()));
        }
    }
    
    // Levenshtein-ish check for close matches
    let known_commands = ["aaa", "afl", "af", "ax", "pdf", "pd", "pdc", "px", "ps", 
                          "i", "ie", "ii", "is", "iz", "s", "?", "help", "q", "quit",
                          "V", "VV", "e", "o", "oo", "w", "x"];
    
    for cmd in known_commands.iter() {
        if input.len() > 0 && cmd.starts_with(&input[0..1]) && input.len() <= cmd.len() + 2 {
            if levenshtein_distance(input, cmd) <= 2 {
                return Some(format!("Did you mean '{}'?", cmd.cyan()));
            }
        }
    }
    
    None
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    
    if len1 == 0 { return len2; }
    if len2 == 0 { return len1; }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 { matrix[i][0] = i; }
    for j in 0..=len2 { matrix[0][j] = j; }
    
    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }
    
    matrix[len1][len2]
}
