//! `ulua-reduce` — command-line Luau test-case reducer (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream
//! `luau-reduce` CLI in CLI/src/Reduce.cpp).

use std::env;

use ulua_reduce_cli::run_main;

fn main() {
  let args: Vec<String> = env::args().collect();
  run_main(&args);
}
