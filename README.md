# OxOS

> ⚠️ **Warning**  
> This operating system is still **experimental** and has only been tested under **very specific conditions**.

**OxOS** is an operating system written in [Rust](https://www.rust-lang.org/), focused on learning, performance, and full architectural control.

## Prerequisites

Before you begin, make sure you have:

## Building
To build the project:

```bash
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-oxos/debug/bootimage-OxOS.bin
```
