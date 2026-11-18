---
module: toolchain
tags: [windows, rust, mingw, linking]
problem_type: environment
---

# Windows Rust linking without MSVC

## Symptoms

`cargo test` failed with `linker link.exe not found` (MSVC) or `unable to find library -lgcc` when mixing LLVM-MinGW with the `windows-gnu` Rust target.

## Root cause

The `x86_64-pc-windows-msvc` target requires Visual Studio Build Tools. The `windows-gnu` target expects a GCC-based MinGW that provides `libgcc` / `libgcc_eh`. LLVM-MinGW alone does not satisfy that expectation.

## What did not work

- Using only LLVM-MinGW `dlltool` with the gnu Rust target
- Relying on rustup alone without a C toolchain on PATH

## Working solution

1. `rustup default stable-x86_64-pc-windows-gnu`
2. Install WinLibs GCC (POSIX UCRT): `BrechtSanders.WinLibs.POSIX.UCRT`
3. Prepend `mingw64/bin` to `PATH` before `cargo test`

## Prevention

Document the Windows toolchain in the root README. Prefer ADR notes when persistence/tooling choices are simplified for demo environments (see ADR-008).
