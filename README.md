# Game Boy Emulator in Rust

This is a simple Game Boy emulator written in Rust. This repository contains an emulator implementation that simulates the Game Boy CPU, GPU, memory bus, input (joypad), timers, and basic instruction set. This works like an noemal Gameboy emulator. You can also use the boot rom file to get the official boot screen (I had to compile it from assembly and then had to convert it to 32kbs). You can use 2 files(boot rom is optional and game rom is mandatory) to run this project. Use the -b for boot romand -r for the game rom when running the program. Note that there are still some problems while running the game but boot screen is getting loaded successfully and it si the main proof of work.

## For Hackclub Reviewer
I want to submit my time from 4 feburary 2026 to 21 feb 2026 to the ysws event Hack Club the Game. Time Spent: 17 hours 54 minutes
I want to submit my time from 21 feburary 2026 to now to the event Horizons. Time Spent: 73 hours 50 minutes

## Features
- CPU emulation (instruction fetching/decoding/execution)
- Memory bus with cartridge ROM loading
- PPU/GPU rendering using `minifb`
- Joypad input handling
- Timer and interrupt emulation
- Optional boot ROM support

## Repository layout
- `emulator/` — main Rust crate for the emulator
	- `Cargo.toml` — crate manifest
	- `src/` — source files
		- `main.rs` — program entry, CLI, and main loop
		- `cpu.rs` — CPU implementation and stepping
		- `memory_bus.rs` — memory map / cartridge handling
		- `gpu.rs` — PPU / rendering and canvas buffer
		- `instruction.rs` — instruction implementations
		- `registers.rs`, `flags_register.rs` — CPU registers and flags
		- `timer.rs` — timer and divider handling
		- `joypad.rs` — input handling
		- `interrupt_flags.rs` — interrupt registers/logic
		- `utils.rs` — helpers

## Prerequisites
- Rust toolchain (stable) and `cargo`: https://rustup.rs

## Build
From the repository root or inside the `emulator` folder:

```bash
cd emulator
cargo build
cd target/debug/
emulator.exe -b dmg_boot_fixed.bin -r porklike.gb
```

## Usage
- Controls: use your system keyboard (mapped in `joypad.rs`) to control the emulated Game Boy. Press `Esc` to exit the emulator window.
