//! `ulua-reduce` — command-line Luau test-case reducer (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream
//! `luau-reduce` CLI in CLI/src/Reduce.cpp).

use ulua_cli_lib::functions::argv::argv;
use ulua_reduce_cli::run_main;

fn main() {
  run_main(&argv());
}
