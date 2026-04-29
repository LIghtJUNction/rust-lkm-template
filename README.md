# Rust Linux Kernel Module Template

A modern template for developing Linux Kernel Modules (LKM) in Rust, built on the [Rust-for-Linux](https://github.com/Rust-for-Linux) project.

## Prerequisites

- Arch Linux (or similar)
- ~50GB disk space for kernel build
- Patience

## Quick Start

### 1. Setup Development Environment

```bash
./scripts/setup -y
```

This will:
- Clone the Rust-for-Linux kernel repository
- Install required dependencies
- Build the kernel with Rust support
- Setup Busybox for initramfs

**Warning**: Kernel compilation takes 30-60+ minutes depending on your hardware.

### 2. Build the Module

```bash
./scripts/build -k /path/to/linux
```

### 3. Run in QEMU

```bash
./scripts/run -k /path/to/linux -b /path/to/busybox
```

Exit QEMU: `Ctrl+A` then `X`

### 4. Hot Reload (Optional)

```bash
cargo install cargo-watch
./scripts/hot-reload -k /path/to/linux -b /path/to/busybox
```

## Project Structure

```
.
├── Cargo.toml          # Rust package metadata
├── Kbuild              # Kernel build configuration
├── Makefile            # Top-level make targets
├── lkm.rs              # Main module entry point
├── module.rs           # Module utilities
├── scripts/
│   ├── build           # Build the LKM
│   ├── run             # Run in QEMU VM
│   ├── hot-reload      # Auto-rebuild on changes
│   └── setup           # Setup dev environment
└── assets/
    └── rust_analyzer_external_modules.patch
```

## Development Notes

### Rust Version

The kernel requires a specific Rust version. The setup script handles this automatically via `rustup override set`.

### Kernel Configuration

The kernel must have these options enabled:
- `CONFIG_RUST=y`
- `CONFIG_RUST_IS_AVAILABLE=y`
- `CONFIG_MODULES=y`

### Building Out-of-Tree Modules

Out-of-tree Rust modules need:
1. A kernel built with `CONFIG_RUST=y`
2. The kernel's `rust/` directory available
3. Proper bindings generated via `bindgen`

## Troubleshooting

### "Rust support is not available"

```bash
cd /path/to/linux
make LLVM=1 rustavailable
```

### Module won't load

Check `dmesg` for errors:
```bash
dmesg | tail -50
```

Common issues:
- Kernel built without `CONFIG_RUST=y`
- Rust toolchain version mismatch
- Missing kernel symbols

## References

- [Rust-for-Linux](https://github.com/Rust-for-Linux)
- [Linux Kernel Rust Documentation](https://docs.kernel.org/rust/)
- [Wu Yu Wei's Blog Post](https://blog.wuYuWei.com/posts/2022/05/04/rust-kernel-module-getting-started/)
