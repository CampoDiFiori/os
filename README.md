
# Pre-requisites

```
rustup override set nightly # for building core and compiler-builtins for target 'none' (see .cargo/config.toml) 
rustup component add rust-src # also for building core, source files as far as I understand
rustup component add llvm-tools-preview # specifically for bootimage tool to work 
cargo install bootimage # a tool that compiles the kernel, bootloader (installed in Cargo.toml) and linkes them
```
