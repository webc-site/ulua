//! `ulua-compile` — command-line Luau source-to-bytecode compiler (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream `luau-compile`
//! CLI in CLI/src/Compile.cpp).

use std::process::exit;

use ulua_cli_lib::functions::argv::argv;
use ulua_compile_cli::run_main;

fn main() {
  exit(run_main(&argv()));
}
