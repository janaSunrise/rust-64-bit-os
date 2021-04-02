# Rust 64-bit OS

  This is an operating system with a kernel supporting x64 bit written from scratch using
  the rust programming language.

## Notes and commands

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
To be able to compile for this target, we need to add it in rustup:

#### Adding target for compilation on systems without OS, for example: bare metals
  - Command: `rustup target add thumbv7em-none-eabihf`

#### Building the target for No-OS systems
  - Command: `cargo build --target thumbv7em-none-eabihf`

#### Linker commands
  - Linux: `cargo rustc -- -C link-arg=-nostartfiles`
  - Windows: `cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"`
  - MacOS: `cargo rustc -- -C link-args="-e __start -static"`

#### In order to recompile these libraries, cargo needs access to the rust source code. The library `rust-src` can be installed using
  - Command: `rustup component add rust-src`

#### Installing bootimage, Adding a `LLVM tools preview` component and using it for running the OS
  - Installation: `cargo install bootimage`
  - Add comp: `rustup component add llvm-tools-preview`
  - Usage: `cargo bootimage`

After executing the command, you should see a bootable disk image named `bootimage-rust-64-bit-os.bin` in your 
`target/x86_64-rust-64-bit-os/debug` directory. You can boot it in a virtual machine or copy it to an USB drive to boot 
it on real hardware. (Note that this is not a CD image, which have a different format, so burning it to a 
CD doesn't work).

### Booting

Command: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust-64-bit-os/debug/bootimage-rust-64-bit-os.bin`

<br />
<div align="center">
  Made by Sunrit Jana with <3
</div>
