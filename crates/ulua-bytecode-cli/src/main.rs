//! `ulua-bytecode` — command-line Luau bytecode disassembler/inspector (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream `luau-bytecode`
//! CLI in CLI/src/Bytecode.cpp).

use std::process::exit;

use ulua_bytecode_cli::run_main;
use ulua_cli_lib::functions::argv::argv;

fn main() {
  exit(run_main(&argv()));
}
