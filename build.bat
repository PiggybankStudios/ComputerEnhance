@echo off

rem CALL :run_ch01_01
CALL :run_ch01_02

goto :eof

:: +==============================+
:: |           ch01_01            |
:: +==============================+
:run_ch01_01
pushd homework\ch01_01
nasm listings\listing37.asm
nasm listings\listing38.asm
cargo run
popd
EXIT /B

:: +==============================+
:: |           ch01_02            |
:: +==============================+
:run_ch01_02
pushd homework\ch01_02
nasm listings\listing39.asm
nasm listings\listing40.asm
cargo run
popd
EXIT /B
