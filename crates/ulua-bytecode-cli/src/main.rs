//! `ulua-bytecode` — command-line Luau bytecode disassembler/inspector (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream `luau-bytecode`
//! CLI in CLI/src/Bytecode.cpp).

ulua_cli_lib::cli_main!(ulua_bytecode_cli::run_main);
