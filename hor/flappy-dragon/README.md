# Flappy Dragon

To compile for Windows On WSL run:

```
sudo apt-get install mingw-w64 
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```
