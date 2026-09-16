//! `ulua-compile` — command-line Luau source-to-bytecode compiler (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream `luau-compile`
//! CLI in CLI/src/Compile.cpp).

use std::{env, process::exit};

use ulua_compile_cli::run_main;

fn main() {
  let args: Vec<String> = env::args().collect();
  exit(run_main(&args));
}
