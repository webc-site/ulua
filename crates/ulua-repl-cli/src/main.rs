//! `ulua` — the Luau REPL / script runner (binary entry point).
//!
//! Thin wrapper over the library `main()`, which marshals argv and dispatches to
//! `repl_main` (faithful port of the upstream `luau` CLI in CLI/src/ReplEntry.cpp).

use ulua_repl_cli::main as run_main;

fn main() {
  run_main();
}
