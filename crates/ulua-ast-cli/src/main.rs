//! `ulua-ast` — Luau AST dump CLI (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream
//! `luau-ast` CLI in CLI/src/Ast.cpp).

ulua_cli_lib::cli_main!(ulua_ast_cli::run_main);
