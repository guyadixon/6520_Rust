# 6502 Emulator & Debugger in Rust

This is a hobby project to emulate the classic MOS 6502 CPU in Rust.

It started as a simple emulator that takes a 64KB binary file as its input, and executes instructions one at a time.

It expanded to be a debugger showing both register values as well as displaying memory.

I then implemented a simple memory mapped 1-bit framebuffer to give me some output.

I then allowed the editing of memory or registers whilst debugging.

It is all command line based, and with clear instructions. Enjoy!
