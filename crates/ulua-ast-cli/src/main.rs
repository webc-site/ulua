//! `ulua-ast` — Luau AST dump CLI (binary entry point).
//!
//! Thin wrapper over the library `main()` (faithful port of the upstream
//! `luau-ast` CLI in CLI/src/Ast.cpp).

use ulua_ast_cli::functions::main::main as lib_main;

fn main() {
  lib_main();
}
