//! `ulua-compile` — command-line Luau source-to-bytecode compiler (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream `luau-compile`
//! CLI in CLI/src/Compile.cpp).

ulua_cli_lib::cli_main!(ulua_compile_cli::run_main);
