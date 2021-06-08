# Cargo

[cargo book](https://doc.rust-lang.org/cargo/index.html)  
[cargo all commands](https://doc.rust-lang.org/cargo/commands/index.html)

**project creation**
cargo new  
cargo init  

cargo check  

**project build, run & install** 
cargo build  
cargo build --release
cargo run  
cargo clean  
cargo tree  
cargo install  

**changing the toolchain**  
cargo +nightly run  
or  
cargo +stable run

## cargo config file

[cargo config docs](https://doc.rust-lang.org/cargo/reference/config.html)

global config file = .cargo/config.toml  
color output in terminal  
alias like  
cargo r for cargo run  
cargo b = cargo build  

**debug and release folder and generated binary**  
for debug build inside debug folder we will have binary of same name as package => when cargo build is invoked  
and in release folder we will have binary of same name as package = when cargo build --release is invoked  

