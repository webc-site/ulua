//! `ulua-analyze` — standalone Luau type-checker CLI (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream
//! `luau-analyze` CLI in CLI/src/Analyze.cpp).

use std::process::exit;

use ulua_analyze_cli::run_main;
use ulua_cli_lib::functions::argv::argv;

fn main() {
  exit(run_main(&argv()));
}
