# Lord OS

My own operating system written in Rust.

I have followed the _Writing an OS in Rust_ series at [os.phil-opp.com](https://os.phil-opp.com)

## Build
To build run:
```
cargo bootimage
```

## Run
To run the application you can use a virtual machine running ubuntu. See this link for instructions regarding installing and using qemu: https://www.minitool.com/partition-disk/qemu-for-windows.html


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
