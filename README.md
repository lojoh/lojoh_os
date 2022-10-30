# Lord OS

My own operating system written in Rust.

I have followed the _Writing an OS in Rust_ series at [os.phil-opp.com](https://os.phil-opp.com)

## Set nightly
Rust has three release channels: stable, beta, and nightly. The Rust Book explains the difference between these channels really well, so take a minute and check it out: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#choo-choo-release-channels-and-riding-the-trains. 

For building an operating system, we will need some experimental features that are only available on the nightly channel, so we need to install a nightly version of Rust.
```
rustup override set nightly
```

## Install deps
```
rustup component add llvm-tools-preview
```

```
cargo install bootimage
```

## Build
To build run:
```
cargo bootimage
```

## Run
To run the application you can use a virtual machine. See this link for instructions regarding installing and using qemu: https://www.minitool.com/partition-disk/qemu-for-windows.html


First step is to copy target/x86_64-blog_os/debug/bootimage-lord_os.bin to D:\qemu\. Then cd to that folder:
```
cd D:\qemu\
```
...and run the file:
```
qemu-system-x86_64 -drive format=raw,file=bootimage-lord_os.bin
```
## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
