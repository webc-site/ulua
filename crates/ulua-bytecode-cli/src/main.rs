//! `ulua-bytecode` — command-line Luau bytecode disassembler/inspector (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream `luau-bytecode`
//! CLI in CLI/src/Bytecode.cpp).

use std::{env, process::exit};

use ulua_bytecode_cli::run_main;

fn main() {
  let args: Vec<String> = env::args().collect();
  exit(run_main(&args));
}
