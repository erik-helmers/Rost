<p align="center">
 <h2 align="center">Rost</h2>
 <p align="center">My first toy OS written in Rust</p>
</p>


This is the begining of a toy OS to learn a bit about system program and more low level-y stuff.

Most of it for the moment is from Philipp Oppermann's [excellent blog](https://os.phil-opp.com/) and most complementary info are from the well known [OSDev site](wiki.osdev.org).

## Features 

- Cooperative multitasking 
- Debug facility  
- Screen printing 
- Serial communication 
- Keyboard support (using cooperative multitasking) 
- Partial PIT support
- Partial RTC support

## Wishlist 
 - Preemptive multithreading
 - ATA driver ? 
 - File system
 - Syscall 
 - Userland programs
 

 ## Install, Build and Run

First off, you _must_ have a nightly rust:
```sh
$ rustup update nightly
$ rustup override nightly 
```

Then you should install the bootimage crate and its dependency:
```sh
$ cargo install bootimage
$ rustup component add llvm-tools-preview
```

You can now run it:
```sh
$ cargo r
```
