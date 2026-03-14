//! Instructions - a gallery of x86 madness

use rand::Rng;
use rand::seq::SliceRandom;

pub struct FakeInstruction {
    pub mnemonic: String,
    pub operands: String,
    pub bytes: String,
    pub size: usize,
}

// Common x86_64 mnemonics
const MNEMONICS_COMMON: &[&str] = &[
    "mov", "lea", "push", "pop", "call", "ret", "jmp",
    "add", "sub", "xor", "and", "or", "not",
    "cmp", "test", "je", "jne", "jz", "jnz", 
    "jg", "jl", "jge", "jle", "ja", "jb",
    "inc", "dec", "mul", "imul", "div", "idiv",
    "shl", "shr", "sar", "sal", "rol", "ror",
    "nop", "int", "syscall", "leave",
    "movzx", "movsx", "cdq", "cqo",
];

const MNEMONICS_RARE: &[&str] = &[
    "cpuid", "rdtsc", "rdrand", "xgetbv",
    "vmread", "vmwrite", "vmptrld", // VM instructions (scary)
    "lock", "rep", "repne",
    "bswap", "bsf", "bsr", "popcnt", "lzcnt",
    "pxor", "movdqa", "movaps", // SIMD
    "endbr64", // CET
    "ud2", // undefined instruction (intentional)
    "hlt", // halt
];

// Register sets
const REGS_64: &[&str] = &["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"];
#[allow(dead_code)]
const REGS_32: &[&str] = &["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp", "esp", "r8d", "r9d", "r10d", "r11d"];
#[allow(dead_code)]
const REGS_8: &[&str] = &["al", "bl", "cl", "dl", "ah", "bh", "ch", "dh", "sil", "dil"];

// Funny immediate values
const FUNNY_IMMEDIATES: &[u64] = &[
    0xDEADBEEF,
    0xCAFEBABE,
    0xBAADF00D,
    0xDEADC0DE,
    0xFEEDFACE,
    0xFACEFEED,
    0xC0FFEE,
    0xBADCAFE,
    0x1337,
    0x420,
    0x69,
    0x666,
    0x42,
    0x7FFFFFFF,
    0x80000000,
];

pub fn generate_instruction(rng: &mut impl Rng, allow_rare: bool) -> FakeInstruction {
    let mnemonic = if allow_rare && rng.r#gen_bool(0.05) {
        *MNEMONICS_RARE.choose(rng).unwrap()
    } else {
        *MNEMONICS_COMMON.choose(rng).unwrap()
    };

    let (operands, size) = generate_operands(rng, mnemonic);
    let bytes = generate_bytes(rng, size);

    FakeInstruction {
        mnemonic: mnemonic.to_string(),
        operands,
        bytes,
        size,
    }
}

fn generate_operands(rng: &mut impl Rng, mnemonic: &str) -> (String, usize) {
    match mnemonic {
        // No operands
        "ret" | "leave" | "nop" | "syscall" | "cdq" | "cqo" | "hlt" | "ud2" | "cpuid" | "rdtsc" => {
            (String::new(), 1)
        }
        
        // Single register
        "push" | "pop" | "inc" | "dec" | "not" | "bswap" => {
            let reg = REGS_64.choose(rng).unwrap();
            (reg.to_string(), if *reg == "rax" || *reg == "rbx" { 1 } else { 2 })
        }
        
        // Register to register
        "mov" | "lea" | "xor" | "and" | "or" | "add" | "sub" | "cmp" | "test" | 
        "movzx" | "movsx" | "imul" => {
            let dst = REGS_64.choose(rng).unwrap();
            
            // Sometimes use funny immediates
            if rng.r#gen_bool(0.15) && (mnemonic == "mov" || mnemonic == "add" || mnemonic == "cmp") {
                let imm = FUNNY_IMMEDIATES.choose(rng).unwrap();
                (format!("{}, 0x{:x}", dst, imm), rng.r#gen_range(5..8))
            } 
            // Sometimes use memory operand
            else if rng.r#gen_bool(0.3) {
                let base = REGS_64.choose(rng).unwrap();
                let offset: i32 = rng.r#gen_range(-0x100..0x100);
                if offset >= 0 {
                    (format!("{}, qword [{} + 0x{:x}]", dst, base, offset), rng.r#gen_range(4..7))
                } else {
                    (format!("{}, qword [{} - 0x{:x}]", dst, base, -offset), rng.r#gen_range(4..7))
                }
            }
            else {
                let src = REGS_64.choose(rng).unwrap();
                (format!("{}, {}", dst, src), rng.r#gen_range(2..4))
            }
        }
        
        // Call/Jump
        "call" | "jmp" => {
            if rng.r#gen_bool(0.5) {
                let target: u64 = rng.r#gen_range(0x400000..0x500000);
                (format!("0x{:x}", target), 5)
            } else {
                let names = ["sym.malloc", "sym.free", "sym.printf", "sym.main", 
                            "fcn.mystery", "loc.loop", "sym.__libc_start_main"];
                (names.choose(rng).unwrap().to_string(), 5)
            }
        }
        
        // Conditional jumps
        "je" | "jne" | "jz" | "jnz" | "jg" | "jl" | "jge" | "jle" | "ja" | "jb" => {
            let target: u64 = rng.r#gen_range(0x400000..0x500000);
            (format!("0x{:x}", target), rng.r#gen_range(2..6))
        }
        
        // Shift operations
        "shl" | "shr" | "sar" | "sal" | "rol" | "ror" => {
            let reg = REGS_64.choose(rng).unwrap();
            let amount: u8 = rng.r#gen_range(1..32);
            (format!("{}, 0x{:x}", reg, amount), 3)
        }
        
        // Multiply/Divide (implicit rax)
        "mul" | "div" | "idiv" => {
            let reg = REGS_64.choose(rng).unwrap();
            (reg.to_string(), 3)
        }
        
        // Int (interrupt)
        "int" => {
            let vectors = [0x3, 0x80, 0x21, 0x10];  // Common interrupt vectors
            let vec = vectors.choose(rng).unwrap();
            (format!("0x{:x}", vec), 2)
        }
        
        // SIMD
        "pxor" | "movdqa" | "movaps" => {
            let xmm = rng.r#gen_range(0..16);
            let xmm2 = rng.r#gen_range(0..16);
            (format!("xmm{}, xmm{}", xmm, xmm2), 4)
        }
        
        // Default: two registers
        _ => {
            let dst = REGS_64.choose(rng).unwrap();
            let src = REGS_64.choose(rng).unwrap();
            (format!("{}, {}", dst, src), rng.r#gen_range(2..5))
        }
    }
}

fn generate_bytes(rng: &mut impl Rng, size: usize) -> String {
    (0..size)
        .map(|_| format!("{:02x}", rng.r#gen::<u8>()))
        .collect::<Vec<_>>()
        .join("")
}
