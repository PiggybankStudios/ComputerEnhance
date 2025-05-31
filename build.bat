@echo off

:: Assemble asm listings into machine code
nasm homework/ch01_01/listings/listing37.asm
nasm homework/ch01_01/listings/listing38.asm


:: Compile Rust-based Homework Solutions
pushd homework\ch01_01
rem cargo build
cargo run
popd
