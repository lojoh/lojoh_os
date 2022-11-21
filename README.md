# Lord OS

My own operating system written in Rust.

I have followed the _Writing an OS in Rust_ series at [os.phil-opp.com](https://os.phil-opp.com)

## Roadmap
- [x] Create operating system kernel
- [x] Implement support for VGA Text Mode
- [x] Create a testing framework
- [x] Handling CPU Exceptions
- [x] Handling Double (and Triple) Faults
- [ ] Set up Interrupt Controller for Hardware Interrupts
- [ ] Memory Management: Paging
- [ ] Memory Management: Support for Heap Allocation
- [ ] Support for Async/Await

## Set nightly
Rust has three release channels: stable, beta, and nightly. The Rust Book explains the difference between these channels really well: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#choo-choo-release-channels-and-riding-the-trains. 

For building an operating system, we will need some experimental features that are only available on the nightly channel, so we need to install a nightly version of Rust.
```
rustup override set nightly
```

## Install dependencies
```
rustup component add llvm-tools-preview
```

```
cargo install bootimage
```

## Build
```
cargo bootimage
```

## Run
To run the application you can use a virtual machine. See this link for instructions regarding installing and using qemu: https://www.minitool.com/partition-disk/qemu-for-windows.html


First step is to copy target/x86_64-lojoh_os/debug/bootimage-lojoh_os.bin to the folder where qemu is running: 
```
copy .\target\x86_64-lord_os\debug\bootimage-lord_os.bin <qemu-location>
```
Then cd to qemu-location:
```
cd <qemu-location>
```
...and run the os:
```
qemu-system-x86_64 -drive format=raw,file=bootimage-lojoh_os.bin
```
## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
