# OxOS

```
 _____              __   ____
/\  __`\          /'__`\/\  _`\
\ \ \/\ \   __  _/\ \/\ \ \,\ \_\
 \ \ \ \ \ /\ \/'\ \ \ \ \/_\__ \
  \ \ \_\ \\/>  </\ \ \_\ \/\ \ \ \
   \ \_____\/\_/\_\\ \____/\ `\____\
    \/_____/\//\/_/ \/___/  \/_____/

----------------------------------------
          OxOS - Versão X.X.XXX
----------------------------------------
```

![image](https://github.com/user-attachments/assets/50eed147-5100-4954-85cd-baa6b9523f61)


> ⚠️ **Experimental Project**
> **OxOS** is a work-in-progress operating system developed for educational and experimental purposes. It has only been tested in **very specific environments** and is not intended for production use.

**OxOS** is a minimal operating system written in [Rust](https://www.rust-lang.org/), [C](), [Assembler]() with a focus on learning, performance, and low-level architectural control.

---

## 🚀 Features

- Bare-metal development in Rust, C e Assembler
- RISC-V 64-bit architecture support
- Full control over boot process and memory
- Simple and clean build system

---

## ✅ Requirements

Before you get started, ensure you have the following:

- A Linux environment or **Windows with WSL**
  [Install WSL](https://learn.microsoft.com/pt-br/windows/wsl/install)
- `rustup`, `cargo`, `gcc`, and `make` installed
- RISC-V toolchain (e.g., `riscv64-unknown-elf-gcc`)


- Rust (riscv64gc-unknown-none-elf)
  - `rustup target add riscv64gc-unknown-none-elf``
- qemu-system-riscv64
---

## 🛠️ Building OxOS

To build the OS:

```bash
sh build.sh
```

This script is tested on **Linux and WSL** environments.

---

## 📦 Output

After building, the kernel binary will be generated in the `build/` directory.

---

## 📚 Learning Resources

OxOS is inspired by:

- [Writing an OS in Rust](https://os.phil-opp.com/)
- [RISC-V Architecture](https://riscv.org/)
- Systems programming and low-level Rust techniques

---

## 🧪 Status

**OxOS** is in the **early stages of development**, currently focusing on setting up the bootloader, basic memory management, and kernel structure.

🔧 **Planned Tasks / Upcoming Features:**

- [X] **VGA text mode** output support
- [ ] Basic **installation BIOS**
- [ ] Process initialization and simple task management
- [ ] Minimalist **file system**
- [ ] Basic hardware drivers (keyboard, disk, etc.)
- [ ] Simple interactive **shell**
- [ ] **Boot logging** system
- [ ] Testing with **QEMU** and, if possible, real hardware
