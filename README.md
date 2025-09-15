# Based Kernel

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Architecture](https://img.shields.io/badge/arch-x86__64-blue.svg)](https://en.wikipedia.org/wiki/X86-64)
[![Bootloader](https://img.shields.io/badge/bootloader-GRUB2-green.svg)](https://www.gnu.org/software/grub/)
[![Build System](https://img.shields.io/badge/build-Cargo%20%7C%20Make-purple.svg)](https://doc.rust-lang.org/cargo/)

*An educational x86-64 kernel with interactive Tic-Tac-Toe game*

[Overview](#-overview) • [Features](#-features) • [Architecture](#-architecture) • [Building](#-building-and-running) • [Demo](#-demonstration) • [Development](#-development-environment) • [Structure](#-project-structure)

</div>

---

## 📖 Overview

This is an educational bare-metal x86-64 kernel implementation that demonstrates fundamental operating system concepts through an interactive gaming experience. Beyond basic kernel functionality, it features a fully playable Tic-Tac-Toe game with keyboard input, sophisticated VGA graphics, and comprehensive interrupt handling.

### Key Characteristics

- **Educational Focus**: Designed for learning kernel development concepts
- **Hybrid Implementation**: Assembly bootloader with Rust kernel logic
- **Cross-Platform Development**: Built on macOS (M3 Max) targeting x86-64
- **Modern Tooling**: Leverages Nix, Docker, and Nushell for development workflow
- **Multiboot2 Compliance**: Standard bootloader interface for compatibility

### Technical Approach

- **Assembly Bootstrap** (`asm/boot.asm`): Hardware initialization and long mode transition
- **Rust Kernel** (`src/main.rs`): Type-safe kernel implementation with `#![no_std]`
- **GRUB2 Loading**: Standard multiboot2 protocol for kernel loading
- **Cross-Compilation**: M3 Max Mac → x86-64 target with Docker-based ISO generation

## 🏛️ Architecture

### Boot Process Flow

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   GRUB2 Stage   │───▶│  Assembly Boot   │───▶│   Rust Kernel   │
│   (Multiboot2)  │    │   (Long Mode)    │    │   (Main Logic)  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
        │                        │                        │
        ▼                        ▼                        ▼
   Loads kernel.bin       32→64 bit transition        VGA text output
   via Multiboot2         Page table setup            Panic handling
   protocol               GDT configuration           Infinite HLT loop
```

### Boot Sequence Details

1. **GRUB2 Initialization**
   - Reads `grub/grub.cfg` configuration
   - Loads `/boot/kernel.bin` from ISO
   - Sets up Multiboot2 environment
   - Transfers control to `_start` in assembly

2. **Assembly Bootstrap** (`asm/boot.asm`)
   - Sets up identity mapping for first 1GB (2MB pages)
   - Configures page tables: PML4 → PDPT → PD
   - Enables PAE (Physical Address Extension)
   - Activates long mode via EFER MSR
   - Sets up minimal GDT for 64-bit operation
   - Far jump to 64-bit code segment
   - Calls Rust `main()` function

3. **Rust Kernel** (`src/main.rs`)
   - Clears VGA text buffer
   - Displays boot messages
   - Enters infinite loop with HLT instruction
   - Handles panics with visual feedback

### Memory Layout

```
Virtual Memory (Identity Mapped):
┌─────────────────┐ 0x0000000000000000
│   Null Page     │ (not mapped)
├─────────────────┤ 0x0000000000001000
│   Low Memory    │ BIOS/IVT region
├─────────────────┤ 0x0000000000100000 (1MB)
│   Kernel Code   │ .text, .rodata, .data, .bss
│   & Data        │ (loaded by GRUB)
├─────────────────┤
│   Available     │ Identity mapped with
│   RAM           │ 2MB pages up to 1GB
└─────────────────┘ 0x0000000040000000 (1GB)

Page Table Structure:
PML4[0] ──▶ PDPT[0] ──▶ PD[0..511]
                        ├─ 0x000000 (2MB page)
                        ├─ 0x200000 (2MB page)
                        ├─ 0x400000 (2MB page)
                        └─ ... (512 × 2MB = 1GB)
```

## 🔨 Building and Running

### Prerequisites

The project supports multiple development environments:

**Option 1: Nix Development Shell (Recommended)**

Required tools:

- Nix Package Manager
- Docker to run `r3dlust/grub2` (image with x86-64 grub2 tools, see [Dockerfile](Dockerfile))

```bash
git clone https://github.com/r3dlust/based-kernel.git
cd based-kernel
nix develop

cargo run
```

**Option 2: Manual Tool Installation**
Required tools:

- Rust nightly toolchain with `rust-src` component and `x86_64-unknown-none` target.
- NASM (x86-64 assembler)
- x86-64 GNU Binutils (`x86_64-unknown-linux-gnu-ld`)
- QEMU (`qemu-system-x86_64`)
- GRUB2 tools (`grub-mkrescue`) or Docker to run `r3dlust/grub2` (image with x86-64 grub2 tools, see [Dockerfile](Dockerfile))

### Build Commands

| Command | Description | Output |
|---------|-------------|--------|
| `cargo run` | Full build → ISO → Run in QEMU | Kernel execution in QEMU |
| `cargo build` | Compile kernel only | `target/x86_64-unknown-none/debug/kernel` |
| `make kernel` | Build via Makefile | Same as `cargo build` |
| `make iso` | Generate bootable ISO | `target/x86_64-unknown-none/release/kernel.iso` |
| `make run` | Build + ISO + QEMU | Kernel execution in QEMU |
| `make clean` | Clean all artifacts | - |

### Build Process Breakdown

1. **Assembly Compilation** (via `build.rs`)

   ```bash
   nasm -f elf64 asm/boot.asm -o boot.o
   ```

2. **Rust Compilation**

   ```bash
   cargo build --target x86_64-unknown-none
   ```

   - Uses custom linker script (`linker.ld`)
   - Links with assembly object file
   - Produces ELF64 kernel binary

3. **ISO Generation** (via `.cargo/run.nu` or Makefile)

   ```bash
   # Create ISO directory structure
   mkdir -p iso/boot/grub
   cp grub/grub.cfg iso/boot/grub/
   cp kernel iso/boot/kernel.bin

   # Generate bootable ISO (via Docker on macOS)
   grub-mkrescue -o kernel.iso iso/
   ```

4. **QEMU Execution**

   ```bash
   qemu-system-x86_64 -cdrom kernel.iso -m 512M -boot d
   ```

## � Project Structure

```
based-kernel/
├── .cargo/                 # Cargo configuration
│   ├── config.toml         # Build settings and target config
│   └── run.nu             # Custom Nushell runner script
├── asm/                   # Assembly source files
│   └── boot.asm           # Multiboot2 header + long mode setup
├── grub/                  # GRUB bootloader configuration
│   └── grub.cfg           # Boot menu settings
├── src/                   # Rust kernel source
│   └── main.rs            # Kernel entry point
├── target/                # Build outputs (generated)
│   └── x86_64-unknown-none/
│       ├── debug/kernel   # Debug kernel binary
│       └── release/kernel # Release kernel binary
├── build.rs               # Build script (NASM integration)
├── Cargo.toml             # Rust project manifest
├── linker.ld              # Custom ELF linker script
├── Makefile               # Alternative build system
├── flake.nix              # Nix development shell
└── Dockerfile             # Docker image for grub-mkrescue
```

### Key Configuration Files

#### Cargo Configuration (`.cargo/config.toml`)

- Sets default target to `x86_64-unknown-none`
- Enables `build-std` for core library compilation from source
- Configures cross-linker (`x86_64-unknown-linux-gnu-ld`)
- Sets custom runner script (`.cargo/run.nu`)
- Disables SSE instructions for kernel compatibility

#### Custom Runner (`.cargo/run.nu`)

- Written in Nushell for cross-platform compatibility
- Automates ISO directory structure creation
- Calls `grub-mkrescue` to generate bootable ISO
- Launches QEMU with appropriate flags
- Cleans up temporary files after execution

#### Linker Script (`linker.ld`)

- Defines memory layout starting at 1MB (above BIOS area)
- Organizes ELF sections: `.multiboot`, `.text`, `.rodata`, `.data`, `.bss`
- Ensures proper alignment for kernel loading
- Sets entry point to `_start` (assembly bootstrap)

## 🛠️ Development Environment

### Nix Development Shell

The `flake.nix` provides a reproducible development environment:

```nix
# Provides:
- Rust nightly toolchain with rust-src
- x86_64-unknown-none target support
- Cross-compilation binutils
- QEMU system emulator
- Docker wrapper for grub-mkrescue
```

### Build Process Details

1. **Assembly Compilation** (`build.rs`)

   ```bash
   nasm -f elf64 asm/boot.asm -o boot.o
   ```

   - Assembles multiboot2 header and boot code
   - Links object file with Rust kernel
   - Ensures proper symbol resolution

2. **Rust Compilation**

   ```bash
   cargo build --target x86_64-unknown-none
   ```

   - Compiles with `#![no_std]` and `#![no_main]`
   - Uses custom linker script via build flags
   - Produces freestanding ELF64 binary

3. **ISO Generation**

   ```bash
   # Create ISO structure
   mkdir -p iso/boot/grub
   cp grub/grub.cfg iso/boot/grub/
   cp kernel iso/boot/kernel.bin

   # Generate bootable ISO
   grub-mkrescue -o kernel.iso iso/
   ```

   - Creates GRUB2-compatible ISO image
   - Uses Docker on macOS for x86_64 grub-mkrescue
   - Produces bootable ISO file

4. **QEMU Testing**

   ```bash
   qemu-system-x86_64 -cdrom kernel.iso -m 512M -boot d -display curses
   ```

   - Boots kernel in x86_64 emulator
   - Uses curses display for terminal compatibility
   - Allocates 512MB RAM for kernel

### Cross-Platform Development Notes

This project was developed on **macOS (M3 Max)** targeting **x86-64**:

- **Rust compilation**: Native cross-compilation works perfectly
- **Assembly**: NASM available on all platforms
- **Linking**: Uses GNU binutils for ELF64 output
- **ISO generation**: Requires x86_64 `grub-mkrescue`, solved with Docker
- **Testing**: QEMU provides cross-architecture emulation

The Docker approach for `grub-mkrescue` eliminates the need for:

- Installing x86_64 GRUB tools on ARM Macs
- Complex cross-compilation of bootloader utilities
- Platform-specific package management issues

## 💡 Implementation Details

### Assembly Bootstrap (`asm/boot.asm`)

**Multiboot2 Header**

```assembly
section .multiboot
align 8
mb2_header_start:
    dd 0xE85250D6                ; Multiboot2 magic number
    dd 0                         ; Architecture: i386
    dd mb2_header_end - mb2_header_start  ; Header length
    dd -(0xE85250D6 + 0 + (mb2_header_end - mb2_header_start))  ; Checksum
```

**Page Table Setup**

- Creates identity mapping for first 1GB using 2MB pages
- Sets up 3-level page table structure (PML4 → PDPT → PD)
- Maps 512 × 2MB pages = 1GB total addressable space

**Long Mode Transition**

1. Enable PAE (Physical Address Extension)
2. Load CR3 with PML4 base address
3. Set LME bit in EFER MSR
4. Enable paging in CR0
5. Load 64-bit GDT
6. Far jump to 64-bit code segment

### Rust Kernel (`src/main.rs`)

**Core Features**

- `#![no_std]` - No standard library dependencies
- `#![no_main]` - Custom entry point (not `main`)
- VGA text mode output at `0xB8000`
- Custom panic handler with visual feedback

**VGA Text Buffer Implementation**

```rust
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// Character format: [ASCII byte][Attribute byte]
// Attribute byte: [Background 4 bits][Foreground 4 bits]
```

**Memory Safety**

- All VGA buffer access wrapped in `unsafe` blocks
- Bounds checking for buffer writes
- Proper pointer arithmetic for character positioning

### Build System Integration

**Cargo + NASM Integration** (`build.rs`)

```rust
// Compile assembly to object file
let nasm_status = Command::new("nasm")
    .args(&["-f", "elf64", "asm/boot.asm", "-o", "boot.o"])
    .status()
    .expect("NASM not found");

// Link assembly object with Rust binary
println!("cargo:rustc-link-arg={}", asm_output.display());
```

**Linker Script** (`linker.ld`)

```ld
ENTRY(_start)                           /* Assembly entry point */
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS {
    . = 1M;                             /* Start at 1MB (above BIOS area) */

    .multiboot : {                      /* Multiboot2 header first */
        KEEP(*(.multiboot))
    }

    .text : { *(.text .text.*) }        /* Code section */
    .rodata : { *(.rodata .rodata.*) }  /* Read-only data */
    .data : { *(.data .data.*) }        /* Initialized data */
    .bss : { *(.bss .bss.*) }           /* Uninitialized data */
}
```

## 🧪 Testing and Debugging

### QEMU Testing

**Basic Execution**

```bash
cargo run  # Automated build + run
make run   # Alternative via Makefile
```

**Manual QEMU Options**

```bash
qemu-system-x86_64 \
    -cdrom kernel.iso \
    -m 512M \
    -boot d \
    -display curses      # Terminal-friendly output
```

### Real Hardware Testing

**Creating Bootable USB** (Advanced)

```bash
# Generate ISO first
make iso

# Write to USB device (replace /dev/sdX with actual device)
sudo dd if=target/x86_64-unknown-none/release/kernel.iso of=/dev/sdX bs=4M status=progress
```

**Hardware Requirements**

- x86-64 processor with long mode support
- At least 512MB RAM
- UEFI or BIOS with legacy boot support
- VGA-compatible graphics for text output

## 📸 Screenshots and Videos

### Boot Process

<img width="1512" height="893" alt="image" src="https://github.com/user-attachments/assets/8b1ed282-484b-4d06-ab2e-c0c3b952d3ef" />

### Kernel Boot

<img width="1512" height="893" alt="image" src="https://github.com/user-attachments/assets/1e2f1706-6103-480a-bd1d-999e464a7a44" />

### Full Game Interaction

https://github.com/user-attachments/assets/cd50f218-ee71-4a0a-82f6-c21cab21af7a

### Development Workflow

<img width="1512" height="833" alt="image" src="https://github.com/user-attachments/assets/1f70e532-10c4-4d9c-8213-0f331e3e15c6" />

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*This project serves as an educational demonstration of kernel development concepts using modern Rust programming practices.*
