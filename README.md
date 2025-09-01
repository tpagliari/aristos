# Aristos

Building a simple OS in Rust.

## Notes while developing
At the moment I am using:
  - The [bootloader][1] to implement a basic BIOS bootloader without C dependencies, only Rust and Assembly.
  - [QEMU][2] to run the disk image

## References
[1]: https://github.com/rust-osdev/bootloader "An experimental pure-Rust x86 bootloader"
[2]: https://www.qemu.org/ "QEMU, A generic and open source machine emulator and virtualizer"

