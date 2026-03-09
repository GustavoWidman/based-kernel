# based-kernel

![license](https://img.shields.io/badge/license-MIT-blue)
![rust](https://img.shields.io/badge/rust-nightly-orange)

an entire operating system, built from scratch just to dodge a c assignment.

this is an educational bare-metal x86-64 kernel featuring a fully playable tic-tac-toe game, vga text graphics, and interrupt-driven keyboard events.

> **want the full story on how i built this?** check out my blog post: [https://blog.r3dlust.com/based-kernel](https://blog.r3dlust.com/based-kernel)

## what is this?

it started as a simple homework assignment: write a 32-bit bootloader in c. i asked my teacher if i could hook it into rust instead, and he said yes. what i didn't realize was that rust doesn't natively support 32-bit bare metal without using custom, undocumented json compiler targets.

so instead of doing the sane thing and writing a json file, i wrote the raw assembly to hijack the boot process, set up 4-level page tables, identity map a gigabyte of memory, and transition the cpu into 64-bit long mode myself. all just to drop into a pure, idiomatic 64-bit rust environment.

## architecture

the whole thing is glued together with rust, assembly, nix, and nushell. an absolutely based stack.

1. **grub2**: loads the kernel via the multiboot2 protocol.
2. **assembly bootstrap (`asm/boot.asm`)**: sets up the page tables, enables physical address extension (pae), flips the magic cpu registers to enter 64-bit mode, and finally jumps to the rust code.
3. **rust kernel (`src/main.rs`)**: takes over from there. it sets up an arena allocator for memory (`talck`), hooks up hardware interrupts for the keyboard to a thread-safe event queue, and runs the actual tic-tac-toe game.

## how to run it

the easiest way to build and run this is using nix. the flake sets up the exact rust nightly toolchain, nasm, qemu, and the docker wrapper needed for `grub-mkrescue` so you don't have to pollute your system.

```bash
git clone https://github.com/r3dlust/based-kernel.git
cd based-kernel

# drop into the dev shell
nix develop

# build the iso and boot it in qemu
cargo run
```

if you hate nix for some reason, you can do it manually. you'll need rust nightly with `rust-src` and the `x86_64-unknown-none` target, nasm, gnu binutils, qemu, and x86_64 grub2 tools (or docker).

```bash
make iso
make run
```

## the game

the kernel doesn't just print "hello world" and halt. it hosts a fully interactive tic-tac-toe game. 

instead of naive polling (checking the keyboard port in an infinite loop like a maniac and wasting cpu cycles), it uses actual hardware interrupts. when you press a key, the cpu pauses, fires an interrupt, and pushes an event to a thread-safe queue. the game loop just sleeps, wakes up to drain the queue, updates the state, and redraws the vga buffer.

## screenshots and videos

### the boot process

<img width="1512" height="893" alt="image" src="https://github.com/user-attachments/assets/8b1ed282-484b-4d06-ab2e-c0c3b952d3ef" />

### the rust kernel taking over

<img width="1512" height="893" alt="image" src="https://github.com/user-attachments/assets/1e2f1706-6103-480a-bd1d-999e464a7a44" />

### gameplay

https://github.com/user-attachments/assets/cd50f218-ee71-4a0a-82f6-c21cab21af7a

### development workflow

<img width="1512" height="833" alt="image" src="https://github.com/user-attachments/assets/1f70e532-10c4-4d9c-8213-0f331e3e15c6" />

### project overview (in pt-br)

https://github.com/user-attachments/assets/e3d63a81-d0fd-4dcf-83b1-d72a1f712422

## license

this project is licensed under the MIT license. read it, understand it, and steal the parts that don't suck.
