# Rust 64-bit OS

This is an operating system with a kernel supporting x64 bit written from scratch using
the rust programming language.

**NOTE**: Do note that this OS is just built for the learning, and testing purpose. It does have
lots of features, but they're not ready to be used in production or in projects. I'll be building
Actual OS once I've learnt, using rust.

## Commands

#### Adding target for compilation on systems without OS, for example: bare metals
  - Command: `rustup target add thumbv7em-none-eabihf`

#### Building the target for No-OS systems
  - Command: `cargo build --target thumbv7em-none-eabihf`

#### Linker commands
  - Linux: `cargo rustc -- -C link-arg=-nostartfiles`
  - Windows: `cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"`
  - MacOS: `cargo rustc -- -C link-args="-e __start -static"`

#### Library `rust-src` to recompile the code, and get access to source code.
  - Command: `rustup component add rust-src`

#### Installing bootimage, Adding a `LLVM tools preview` component and for running the OS
  - Installation: `cargo install bootimage`
  - Add comp: `rustup component add llvm-tools-preview`
  - Usage: `cargo bootimage`

### Booting

Command: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust-64-bit-os/debug/bootimage-rust-64-bit-os.bin`

## Notes

### Building for bare metal target

<details>
<summary>Building for bare metal target</summary>

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

</details>

### The Boot Process

<details>
<summary>The BOOT Process</summary>

When you turn on a computer, it begins executing firmware code that is stored in motherboard ROM. 
This code performs a power-on self-test, detects available RAM, and pre-initializes the CPU and hardware. 
Afterwards it looks for a bootable disk and starts booting the operating system kernel.

On x86, there are two firmware standards: the "Basic Input/Output System" (BIOS) and the newer "Unified Extensible Firmware Interface" (UEFI). The BIOS standard is old and outdated, but simple and well-supported on any x86 machine since the 1980s. UEFI, in contrast, is more modern and has much more features, but is more complex to set up

</details>

#### BIOS BOOT

<details>
<summary>BIOS BOOT</summary>

Almost all x86 systems have support for BIOS booting, including newer UEFI-based machines that use an emulated BIOS. This is great, because you can use the same boot logic across all machines from the last centuries. But this wide compatibility is at the same time the biggest disadvantage of BIOS booting, because it means that the CPU is put into a 16-bit compatibility mode called real mode before booting so that archaic bootloaders from the 1980s would still work.

</details>

#### Multiboot standard


<details>
<summary>MultiBoot Standard</summary>

To avoid that every operating system implements its own bootloader, which is only compatible with a single OS, the Free Software Foundation created an open bootloader standard called Multiboot in 1995. The standard defines an interface between the bootloader and operating system, so that any Multiboot compliant bootloader can load any Multiboot compliant operating system. The reference implementation is GNU GRUB, which is the most popular bootloader for Linux systems.

To make a kernel Multiboot compliant, one just needs to insert a so-called Multiboot header at the beginning of the kernel file. This makes it very easy to boot an OS in GRUB. However, GRUB and the Multiboot standard have some problems too:

- They support only the 32-bit protected mode. This means that you still have to do the CPU configuration to switch to the 64-bit long mode.

- They are designed to make the bootloader simple instead of the kernel. For example, the kernel needs to be linked with an adjusted default page size, because GRUB can't find the Multiboot header otherwise. Another example is that the boot information, which is passed to the kernel, contains lots of architecture dependent structures instead of providing clean abstractions.

- Both GRUB and the Multiboot standard are only sparsely documented.

- GRUB needs to be installed on the host system to create a bootable disk image from the kernel file. This makes development on Windows or Mac more difficult.


</details>

### VGA Text Buffer


<details>
<summary>VGA Text Buffer</summary>

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

</details>

### CPU Exceptions

<details>
<summary>CPU Exceptions</summary>

An exception signals that something is wrong with the current instruction. For example, the CPU issues an exception if the current instruction tries to divide by 0. When an exception occurs, the CPU interrupts its current work and immediately calls a specific exception handler function, depending on the exception type.

On x86 there are about 20 different CPU exception types. The most important are:

- Page Fault: A page fault occurs on illegal memory accesses. For example, if the current instruction tries to read from an unmapped page or tries to write to a read-only page.

- Invalid Opcode: This exception occurs when the current instruction is invalid, for example when we try to use newer SSE instructions on an old CPU that does not support them.

- General Protection Fault: This is the exception with the broadest range of causes. It occurs on various kinds of access violations such as trying to execute a privileged instruction in user level code or writing reserved fields in configuration registers.

- Double Fault: When an exception occurs, the CPU tries to call the corresponding handler function. If another exception occurs while calling the exception handler, the CPU raises a double fault exception. This exception also occurs when there is no handler function registered for an exception.

- Triple Fault: If an exception occurs while the CPU tries to call the double fault handler function, it issues a fatal triple fault. We can't catch or handle a triple fault. Most processors react by resetting themselves and rebooting the operating system.

For the full list of exceptions check out the [OSDev wiki](https://wiki.osdev.org/Exceptions).

</details>

#### Double Fault

<details>
<summary>What is a double fault?</summary>

In simplified terms, a double fault is a special exception that occurs when the CPU fails to invoke an exception handler. 
For example, it occurs when a page fault is triggered but there is no page fault handler registered in the Interrupt Descriptor Table (IDT). 
So it's kind of similar to catch-all blocks in programming languages with exceptions, e.g. `catch(...)` in C++ or `catch(Exception e)` in Java or C#.

A double fault behaves like a normal exception. It has the vector number `8` and we can define a normal handler function for it in the IDT. It is really important to provide a double fault handler, because if a double fault is unhandled a fatal triple fault occurs. Triple faults can't be caught and most hardware reacts with a system reset.

</details>

**Thanks to [Phil OPP](https://github.com/phil-opp) for these valuable notes!**

<br />
<div align="center">
  Made by Sunrit Jana with <3
</div>
