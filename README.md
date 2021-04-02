# Rust 64-bit OS

  This is an operating system with a kernel supporting x64 bit written from scratch using
  the rust programming language.

## Commands

### Adding target for compilation on systems without OS, for example: bare metals
  - Command: `rustup target add thumbv7em-none-eabihf`

### Building the target for No-OS systems
  - Command: `cargo build --target thumbv7em-none-eabihf`

### Linker commands
  - Linux: `cargo rustc -- -C link-arg=-nostartfiles`
  - Windows: `cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"`
  - MacOS: `cargo rustc -- -C link-args="-e __start -static"`

### Library `rust-src` to recompile the code, and get access to source code.
  - Command: `rustup component add rust-src`

#### Installing bootimage, Adding a `LLVM tools preview` component and for running the OS
  - Installation: `cargo install bootimage`
  - Add comp: `rustup component add llvm-tools-preview`
  - Usage: `cargo bootimage`

### Booting

Command: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust-64-bit-os/debug/bootimage-rust-64-bit-os.bin`

## Notes

### Building for bare metal target
By default Rust tries to build an executable that is able to run in your current system environment. 
For example, if you're using Windows on `x86_64`, Rust tries to build a `.exe` Windows executable that 
uses `x86_64` instructions.
This environment is called your "host" system.

To describe different environments, Rust uses a string called target triple. You can see the target triple for your host system by running `rustc --version --verbose`

By compiling for our host triple, the Rust compiler and the linker assume that there is an underlying operating system such as Linux or Windows that use the C runtime by default, which causes the linker errors. So to avoid the linker errors, we can compile for a different environment with no underlying operating system.

An example for such a bare metal environment is the `thumbv7em-none-eabihf` target triple, which describes an 
[embedded](https://en.wikipedia.org/wiki/Embedded_system) [ARM](https://en.wikipedia.org/wiki/ARM_architecture) system. 
The details are not important, all that matters is that the target triple has no underlying operating system, 
which is indicated by the none in the target triple. 
To be able to compile for this target, we need to add it in rustup.

### The Boot Process

When you turn on a computer, it begins executing firmware code that is stored in motherboard ROM. 
This code performs a power-on self-test, detects available RAM, and pre-initializes the CPU and hardware. 
Afterwards it looks for a bootable disk and starts booting the operating system kernel.

On x86, there are two firmware standards: the "Basic Input/Output System" (BIOS) and the newer "Unified Extensible Firmware Interface" (UEFI). The BIOS standard is old and outdated, but simple and well-supported on any x86 machine since the 1980s. UEFI, in contrast, is more modern and has much more features, but is more complex to set up

### VGA Text Buffer

To print a character to the screen in VGA text mode, one has to write it to the text buffer of the VGA hardware. The VGA text buffer is a two-dimensional array with typically 25 rows and 80 columns, which is directly rendered to the screen. Each array entry describes a single screen character through the following format:

| Bit(s)| Value            |
|-------|------------------|
| 0-7   | ASCII code point |
| 8-11  | Foreground color |
| 12-14 | Background color |
| 15    | Blink            |

The first byte represents the character that should be printed in the ASCII encoding. To be exact, it isn't exactly ASCII, but a character set named code page 437 with some additional characters and slight modifications. For simplicity, we proceed to call it an ASCII character in this post.

The second byte defines how the character is displayed. The first four bits define the foreground color, the next three bits the background color, and the last bit whether the character should blink. The following colors are available:

| Number | Color      | Number + Bright Bit | Bright Color |
|--------|------------|---------------------|--------------|
| 0x0    | Black      | 0x8                 | Dark Gray    |
| 0x1    | Blue       | 0x9                 | Light Blue   |
| 0x2    | Green      | 0xa                 | Light Green  |
| 0x3    | Cyan       | 0xb                 | Light Cyan   |
| 0x4    | Red        | 0xc                 | Light Red    |
| 0x5    | Magenta    | 0xd                 | Pink         |
| 0x6    | Brown      | 0xe                 | Yellow       |
| 0x7    | Light Gray | 0xf                 | White        |


<br />
<div align="center">
  Made by Sunrit Jana with <3
</div>
