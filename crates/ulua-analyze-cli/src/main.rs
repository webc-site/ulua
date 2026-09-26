//! `ulua-analyze` — standalone Luau type-checker CLI (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream
//! `luau-analyze` CLI in CLI/src/Analyze.cpp).

ulua_cli_lib::cli_main!(ulua_analyze_cli::run_main);
