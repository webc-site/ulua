//! `ulua-reduce` — command-line Luau test-case reducer (binary entry point).
//!
//! Thin wrapper over the library `run` (faithful port of the upstream `luau-reduce`
//! CLI in CLI/src/Reduce.cpp). `run` 自管退出码（`help()` 内联 `exit(1)`），故走
//! `unit` 形。

ulua_cli_lib::cli_main!(unit: ulua_reduce_cli::run_main);
