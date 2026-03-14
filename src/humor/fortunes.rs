//! Fortunes - wisdom for the ages (or at least for the session)

use rand::seq::SliceRandom;

const FORTUNES: &[&str] = &[
    // Radare2 parodies
    "In Soviet Russia, binary reverses you!",
    "Welcome to IDA 15.0 (just kidding, you can't afford it)",
    "No disassemble! (please?)",
    "May the segfault be with you",
    "All your base are belong to r2",
    "AHHHHH!!!! ASSEMBLY CODE!!!!!! HOLD ME I'M SCARED!!!",
    "Documentation? We don't do that here.",
    "The real malware was the friends we made along the way",
    
    // C/C++ humor
    "Segmentation fault is not a city in California",
    "undefined behavior: the spice of life",
    "Remember: free() twice for extra freedom",
    "If it compiles, ship it",
    "goto considered harmful, but I like to live dangerously",
    "Trust me, I'm a compiler",
    "Memory leaks build character",
    "const correctness is just a suggestion",
    "void* is just a pointer with commitment issues",
    "The compiler optimized away your hopes and dreams",
    
    // Assembly jokes
    "Real programmers use butterflies (xkcd 378)",
    "mov rax, existential_crisis",
    "push problems; call therapist; pop excuses",
    "xor eax, eax  ; the most elegant zero",
    "jmp 0x666  ; to hell we go",
    "nop nop nop nop nop nop ; this is fine",
    "ret ; from this conversation",
    
    // Debugging wisdom
    "Have you tried turning it off and never on again?",
    "printf debugging: 90% of all bugs, 100% of the time",
    "Works on my machine ¯\\_(ツ)_/¯",
    "The bug is not in the code, it's in your soul",
    "Breakpoints are just naps for CPUs",
    "Core dumped. So did my motivation.",
    "Stack smashing detected. The stack had it coming.",
    
    // Career advice
    "Have you considered a career in farming?",
    "Reverse engineering: for when you've already given up on life",
    "There's always grad school",
    "At least you're not writing JavaScript... wait, are you?",
    "The real treasure was the bugs we shipped along the way",
    "Your impostor syndrome is well-founded",
    "This meeting could've been an email",
    
    // Self-aware humor
    "This tool is held together by duck tape and prayers",
    "Bugs? No, those are surprise features",
    "Not a bug, not a feature, just vibes",
    "The source code is the documentation",
    "git blame: for when you need to know who to fire",
    "We only have bugs, features are an unintended side-effect",
    "Help subcommand will eventually be implemented",
    
    // Matrix/Movie references
    "Follow the white rabbit... into the stack",
    "Hello, Mr. Anderson",
    "There is no spoon, only undefined behavior",
    "I know kung fu... and x86 assembly",
    "The One will balance the stack, or so the prophecy says",
    
    // Misc nerd humor
    "I am Pentium of Borg. Division is futile. You will be approximated.",
    "There are only 10 types of people: those who understand binary and those who don't",
    "A SQL query walks into a bar, walks up to two tables and asks 'Can I join you?'",
    "Why do programmers prefer dark mode? Because light attracts bugs.",
    "Algorithm: a word used by programmers when they don't want to explain what they did",
    "sleep(8 * 60 * 60); // TODO: implement human.rest()",
    "// TODO: fix this before release - comment from 2003",
    "rm -rf / (just kidding... unless?)",
    
    // Crypto/Security
    "Security through obscurity: because nobody can hack what nobody understands",
    "0xDEADBEEF: the forbidden snack",
    "0xCAFEBABE: Java's contribution to hexspeak",
    "Your password is in plaintext somewhere, I guarantee it",
    "Two-factor authentication: because your password is definitely '123456'",
    "Encryption is just ROT13 applied twice... right? RIGHT?",
    "CTF player by day, impostor by night",
    
    // File format jokes
    "ELF: Executable and Linkable Format (also: Erratic, Lacking, Frustrating)",
    "PE files: Probably Evil",
    "Mach-O: for when you want ELF but different",
    "PDF is just a fancy zip file (please don't @ me)",
    
    // Existential
    "What is code but misery persevering?",
    "Every program ends with exit(0), or panic, or heat death of the universe",
    "In the beginning there was machine code, and it was good. Then came abstractions.",
    "Debugging is like being the detective in a crime movie where you're also the murderer",
    "The bug was inside you all along",
    "I think, therefore I segfault",
    
    // System programming
    "Kernel panic: when the kernel realizes what you've done",
    "Ring 0: trust nobody, not even yourself",
    "syscall: please do the thing, kernel-senpai",
    "mmap: because you deserve to suffer",
    "fork(): now there are two of them. This is getting out of hand.",
    
    // Compiler humor
    "The compiler is not your enemy. (The compiler is definitely your enemy.)",
    "Optimizations: turning your code into something you don't recognize",
    "-O3: 'I too like to live dangerously'",
    "Clang: the friendly compiler (that still hates you)",
    "GCC: Great Compiler Collection (of incomprehensible error messages)",
    
    // Reversing specific
    "IDA: Interactive Disassembler (and subscription to poverty)",
    "Ghidra: because the NSA wants you to succeed (suspicious)",
    "Binary Ninja: for when you have more money than sense (but less than IDA)",
    "radare2: the CLI for people who hate themselves",
    "rr2: radare2 but make it ✨ rusty ✨",
    "Decompilers lie. Disassemblers lie. The binary lies. Trust nothing.",
    "Anti-debug? More like anti-fun.",
    "VMProtect: because someone hates you specifically",
    "Unpacking a binary is like opening a Russian nesting doll of suffering",
];

pub fn get_random() -> &'static str {
    let mut rng = rand::thread_rng();
    FORTUNES.choose(&mut rng).unwrap_or(&"Fortune not found. This is a bad omen.")
}

#[allow(dead_code)]
pub fn get_all() -> &'static [&'static str] {
    FORTUNES
}
