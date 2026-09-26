//! `ulua-ast` — Luau AST dump CLI (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream
//! `luau-ast` CLI in CLI/src/Ast.cpp).

use std::process::exit;

use ulua_ast_cli::run_main;
use ulua_cli_lib::functions::argv::argv;

fn main() {
  exit(run_main(&argv()));
}
