//! Easter eggs - hidden gems for the curious

use colored::*;

pub fn check_easter_egg(input: &str) -> Option<String> {
    let input_lower = input.to_lowercase().trim().to_string();
    
    match input_lower.as_str() {
        // Classic Unix humor
        "make me a sandwich" => Some(
            "What? Make it yourself.".to_string()
        ),
        "sudo make me a sandwich" => Some(
            "Okay. 🥪\n(You're still not root though)".to_string()
        ),
        "sudo" => Some(
            "sudo: permission denied\n(Nice try, you're not in the sudoers file. This incident will be reported.)".to_string()
        ),
        "rm -rf /" | "rm -rf /*" | "sudo rm -rf /" => Some(
            "Nice try. 🙄\n(But seriously, don't do that)".to_string()
        ),
        ":(){ :|:& };:" => Some(
            "Fork bomb detected! 💣\nJust kidding, we're not that stupid.\n(Please don't actually run that anywhere)".to_string()
        ),
        
        // r2 classic commands that we parody
        "rotpid" => Some(
            "Process rotated successfully!\n(Just kidding, what does that even mean?)".to_string()
        ),
        "aaaa" | "aaaaa" | "aaaaaa" => Some(
            format!("{}",
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\nOkay okay, I get it. You want LOTS of analysis.\n(It's still fake though)".bright_yellow()
            )
        ),
        
        // Hidden commands
        "hello" | "hi" | "hey" => Some(
            "Hello there! 👋\nYou found a friendly message. Savor it, there aren't many.".to_string()
        ),
        "42" => Some(
            "The answer to life, the universe, and everything.\nBut what was the question? (It was probably about segfaults)".to_string()
        ),
        "1337" | "leet" => Some(
            "H4CK3R D3T3CT3D!\nJust kidding. But your l33t sk1llz are noted.".to_string()
        ),
        "password" | "password123" | "admin" => Some(
            "Did you really just type that? 🤦\nI'm not even a login prompt.".to_string()
        ),
        
        // Star Wars
        "may the force be with you" => Some(
            "And also with you.\nMay your pointers be valid and your buffers never overflow.".to_string()
        ),
        "i am your father" => Some(
            "NOOOOOOOO!\n*jumps into cloud city air shaft*".to_string()
        ),
        "these are not the droids" => Some(
            "These are not the bytes you are looking for.\n*waves hand mysteriously*".to_string()
        ),
        
        // Matrix
        "red pill" => Some(
            "You stay in Wonderland, and I show you how deep the stack overflow goes.".to_string()
        ),
        "blue pill" => Some(
            "The story ends. You wake up in your bed and believe whatever you want about memory safety.".to_string()
        ),
        "there is no spoon" => Some(
            "There is no documentation either, but here we are.".to_string()
        ),
        
        // Programmer jokes
        "why" => Some(
            "Because someone thought it would be funny.\nThey were right.".to_string()
        ),
        "help me" => Some(
            "I'm trying! But honestly, you're beyond help.\n(Type '?' for actual help. It won't be useful, but it exists.)".to_string()
        ),
        "im stuck" | "i'm stuck" | "i am stuck" => Some(
            "Have you tried:\n• Reading the (nonexistent) documentation?\n• Turning it off and on again?\n• Crying?\n• All of the above?".to_string()
        ),
        "fix my code" => Some(
            "No.\nBut I can add more bugs for free.".to_string()
        ),
        
        // Hexspeak checks - moved to special handling
        "0xdeadbeef" => Some(
            format!("{}\n{}\n{}",
                "🥩 DEAD BEEF DETECTED 🥩".bright_red().bold(),
                "The forbidden hexadecimal snack!",
                "This is what happens when programmers get hungry."
            )
        ),
        "0xcafebabe" => Some(
            format!("{}\n{}\n{}",
                "☕ CAFE BABE DETECTED ☕".bright_magenta().bold(),
                "Ah, the Java magic number!",
                "Write once, debug everywhere™"
            )
        ),
        "0xbaadf00d" => Some(
            format!("{}\n{}",
                "🍔 BAAD FOOD DETECTED 🍔".yellow().bold(),
                "Uninitialized heap memory never tasted so... corrupted."
            )
        ),
        "0xfeedface" => Some(
            format!("{}\n{}",
                "👤 FEED FACE DETECTED 👤".cyan().bold(),
                "Mach-O magic number! Your binary is hungry for execution."
            )
        ),
        
        // Misc
        "panic" => Some(
            "KERNEL PANIC - NOT SYNCING\n\njust kidding lol\n\n(but imagine if it was real 😱)".to_string()
        ),
        "crash" => Some(
            format!("{}", 
                "Segmentation fault (core dumped)\n\nAre you happy now? (The core wasn't actually dumped, I'm not that committed to the bit)".dimmed()
            )
        ),
        "hack" => Some(
            "🎩 H A C K E R M A N 🎩\nYou are now accessing the mainframe.\n(You're not. You're running a joke tool.)".to_string()
        ),
        "exploit" => Some(
            "Exploit development requires:\n• Patience\n• Coffee\n• Questionable life choices\n• More coffee\n\nYou have: this tool (insufficient)".to_string()
        ),
        "flag" | "ctf" => Some(
            "🚩 FLAG{y0u_f0und_th3_34st3r_3gg} 🚩\n(This won't work on real CTFs, sorry)".to_string()
        ),
        
        // Nihilism
        "meaning of life" => Some(
            "42, obviously.\nAlternatively: pointers, pain, and printf debugging.".to_string()
        ),
        "what is my purpose" => Some(
            "You pass butter.\n(You reverse binaries. Same thing, really.)".to_string()
        ),
        
        // Credits
        "credits" | "about" => Some(format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            "╔════════════════════════════════════════╗".bright_cyan(),
            "║         rr2 - radare rust 2            ║".bright_cyan(),
            "║  'where undefined behavior is a feature'║".bright_cyan(),
            "╠════════════════════════════════════════╣".bright_cyan(),
            "║  Made with 💀 by reverse engineers     ║".bright_cyan(),
            "╚════════════════════════════════════════╝".bright_cyan(),
        )),
        
        "version" | "ver" => Some(format!(
            "{}\n{}\n{}\n{}",
            "rr2 version 0.1.337",
            "Build: Schrödinger's release (both stable and unstable)",
            "Commit: 0xDEADC0DE",
            "License: WTFPL (Do What The F*** You Want To Public License)"
        )),
        
        // Konami code (text version)
        "up up down down left right left right b a" | "uuddlrlrba" => Some(
            "🎮 KONAMI CODE ACTIVATED! 🎮\n+30 lives (to your debugging session)\nInfinite continues (because you'll need them)".to_string()
        ),
        
        _ => None,
    }
}

pub fn check_address_easter_egg(addr: u64) -> Option<String> {
    match addr {
        0xDEADBEEF => Some("You seek the forbidden beef. 🥩".to_string()),
        0xCAFEBABE => Some("Java? In MY radare? It's more likely than you think. ☕".to_string()),
        0xBAADF00D => Some("Mmm, uninitialized heap memory. Delicious. 🤢".to_string()),
        0xDEADC0DE => Some("The code is dead. Long live the code. 💀".to_string()),
        0xFEEDFACE => Some("Mach-O says: feed me binaries! 👤".to_string()),
        0xFACEFEED => Some("The inverse of normal. How contrarian. 🔄".to_string()),
        0x8BADF00D => Some("iOS watchdog says: your app took too long. ⏱️".to_string()),
        0x1BADB002 => Some("Multiboot magic! GRUB was here. 🥾".to_string()),
        0x666 => Some("Seeking the number of the beast. 😈".to_string()),
        0x1337 => Some("l33t address detected. Very elite. Much hacker. 🎩".to_string()),
        0x420 => Some("Nice. 🌿".to_string()),
        0x69 => Some("Nice. 😏".to_string()),
        0x7FFFFFFF => Some("INT_MAX! Living on the edge. 📈".to_string()),
        0x80000000 => Some("INT_MIN! The abyss stares back. 📉".to_string()),
        _ => None,
    }
}
