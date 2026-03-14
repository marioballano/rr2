# rr2 — Rust Reverse Engineering Framework

```
                 _____ 
 _ __ _ __ ___  |___ / 
| '__| '__/ _ \   |_ \ 
| |  | | |  __/  ___) |
|_|  |_|  \___| |____/ 
```

**rr2** is a next-generation, memory-safe binary analysis framework written entirely in Rust. It delivers comprehensive reverse engineering capabilities — including multi-format binary parsing, deep code analysis, disassembly, decompilation, and an interactive REPL — at a level of performance and correctness that legacy C-based tooling fundamentally cannot achieve.

rr2 is the result of years of research into static analysis, compiler theory, and systems programming. It is not a hobby project.

---

## Key Capabilities

- **Universal binary parsing** — Native support for ELF, PE, Mach-O, and Fat binary formats via a zero-copy parsing pipeline
- **Deep code analysis** — Full function boundary detection, basic block reconstruction, cross-reference resolution, cyclomatic complexity measurement, and stack frame analysis
- **Disassembly engine** — High-throughput x86_64 disassembly with semantic annotation and control flow awareness
- **Decompilation** — Structured pseudocode generation from raw machine code, suitable for rapid comprehension of unknown binaries
- **Hex inspection** — Annotated hex views with automatic detection of known binary signatures and magic constants
- **String analysis** — Extraction and classification of embedded strings across all binary sections
- **Interactive REPL** — A persistent, stateful shell with command history, scripting support, and a composable command grammar
- **Visual mode** — Terminal-native control flow graph and binary layout visualization
- **Contextual diagnostics** — All error and analysis output includes actionable, human-readable context

---

## Installation

**Requirements:** Rust 1.75 or later.

```bash
git clone https://github.com/marioballano/rr2 
cd rr2
cargo build --release
```

The resulting binary is fully self-contained with no runtime dependencies.

```bash
./target/release/rr2 --help
```

---

## Usage

```bash
# Open a binary for analysis
rr2 /path/to/binary

# Execute a single command and exit
rr2 -c "aaa; afl" /path/to/binary

# Suppress startup output
rr2 -q /path/to/binary

# Open with write permissions
rr2 -w /path/to/binary

# Override architecture hint
rr2 -a x86 -b 64 /path/to/binary
```

---

## Command Reference

### Analysis

| Command | Description |
|---------|-------------|
| `aaa` | Full analysis pass: functions, basic blocks, xrefs, strings, stack frames |
| `aa` | Standard function analysis |
| `afl` | List all identified functions with metadata |
| `ax` | List all cross-references |
| `axt [addr]` | List references pointing to address |
| `axf [addr]` | List references originating from address |

### Disassembly & Decompilation

| Command | Description |
|---------|-------------|
| `pd [N]` | Disassemble N instructions from current position |
| `pdf` | Disassemble the function at current position |
| `pdc` | Emit decompiled pseudocode for current function |

### Print & Inspection

| Command | Description |
|---------|-------------|
| `px [N]` | Hex dump of N bytes from current position |
| `ps` | Print string at current position |
| `iz` | List all strings found in the binary |
| `i` | Display binary metadata (format, arch, entry point, sections, symbols) |

### Navigation

| Command | Description |
|---------|-------------|
| `s [addr]` | Seek to address or symbol |
| `s+N` / `s-N` | Seek forward or backward by N bytes |

### Visual

| Command | Description |
|---------|-------------|
| `V` | Enter visual inspection mode |
| `VV` | Enter control flow graph view |

### Session

| Command | Description |
|---------|-------------|
| `e [key=val]` | Get or set configuration |
| `?` | Display help |
| `q` | Exit |

---

## Sample Session

```
$ rr2 /usr/bin/ssh

[0x00004a20:r/w]> i

╔══════════════════════════════════════════════════════════════╗
║                    BINARY INFORMATION                        ║
╠══════════════════════════════════════════════════════════════╣
║  Type:           ELF64                                    ║
║  Architecture:   x86_64                                   ║
║  Bits:           64                                       ║
║  Endian:         little                                   ║
║  Entry:          0x00004a20                               ║
║  Size:           892416 bytes (871.50 KB)                 ║
║  Sections:       28                                       ║
║  Symbols:        stripped                                 ║
║  Debug Info:     no                                       ║
║  PIE:            yes (ASLR enabled)                       ║
╚══════════════════════════════════════════════════════════════╝

[0x00004a20:r/w]> aaa
[afr] Scanning for functions... found 312 functions
[afbr] Detecting basic blocks... done
[axt] Resolving cross-references... 1847 xrefs resolved
[afsr] Analyzing stack frames... done
[izzr] Extracting strings... 204 strings
[afcc] Computing cyclomatic complexity... average: 14
Analysis complete.

[0x00004a20:r/w]> pd 4
    0x00004a20  4889e5           mov          rbp, rsp
    0x00004a23  4883ec30         sub          rsp, 0x30
    0x00004a27  897ddc           mov          dword [rbp-0x24], edi
    0x00004a2a  488975d0         mov          qword [rbp-0x30], rsi

[0x00004a20:r/w]> q
```

---

## Architecture

rr2 is structured as a set of composable, independently testable modules:

| Module | Responsibility |
|--------|----------------|
| `core` | Shell state, configuration, REPL loop |
| `cli` | Command parsing, dispatch, help system |
| `binary` | Format detection and metadata extraction |
| `analysis` | Function analysis, xref resolution, complexity metrics |
| `disasm` | Instruction decoding and annotation |
| `print` | Hex, string, and structured output rendering |
| `visual` | Terminal UI and graph rendering |

---

## Performance

rr2 is built with Rust's ownership model at its core. There are no garbage collection pauses, no use-after-free conditions, and no data races. The analysis pipeline is designed for low-latency interactive use and scales linearly with binary size.

Benchmarks available on request.

---

## License

**rr2 Proprietary Software License, Version 1.0**

Copyright © 2026. All rights reserved.

This software and all associated source code, documentation, binaries, and related materials ("Software") are the exclusive proprietary property of the copyright holder. No rights are granted except as explicitly stated below.

**YOU MAY NOT**, without prior written permission from the copyright holder:
- Copy, reproduce, or distribute this Software or any portion thereof
- Modify, adapt, translate, or create derivative works based on this Software
- Reverse engineer, disassemble, or decompile this Software (the irony is noted)
- Sublicense, sell, rent, lease, or otherwise transfer rights to this Software
- Use this Software in any production environment, commercial or otherwise
- Use this Software for any purpose other than personal, non-commercial evaluation

**YOU MAY**, solely for personal evaluation purposes:
- Build and run this Software on a single machine you own and control
- Read the source code for the purpose of understanding its design

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE FOR ANY CLAIM, DAMAGES, OR OTHER LIABILITY ARISING FROM THE USE OF THIS SOFTWARE. THIS INCLUDES BUT IS NOT LIMITED TO: corrupted binaries, incorrect analysis, career-ending security assessments, existential crises triggered by the decompiler output, and any general confusion about what this tool actually does.

Violation of this license may result in civil and criminal penalties to the fullest extent permitted by applicable law, including but not limited to strongly-worded emails.

---

## Disclaimer

rr2 analysis output is generated using proprietary heuristics. Results should be treated as approximate. The decompiler in particular operates on a best-effort basis. Do not make security-critical decisions based solely on rr2 output without independent verification.

The fortune system is not affiliated with any standardized threat intelligence feed.
