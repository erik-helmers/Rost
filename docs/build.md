# How rost is built ?

(## Requirements

What we "have": 
    - a Rust crate (just `cargo build` to build it)
    - a bootloader written in asm and a linker script 
What we want: 
    -  a bootable iso files )

## Solution

Chosing the build went as follows: 
- Cargo alone is not sufficient because [it isn't designed for that](https://github.com/rust-lang/rfcs/pull/1777#issuecomment-256335815)
- CMake could have been nice, but.. it's not made for Rust and/or ASM at all. 
- Make is pretty simple, and match what we need and want.

Now we want to call the Rust code from ASM
 > Should we compile the Rust code as a staticlib or as an executable

I prefer to avoid bash scripts directly.
