//! 各 CLI bin 入口（`main.rs`）的同形样板收口（`cli_preamble` 先例的入口侧
//! 补全）：`fn main() { exit(run_main(&argv())); }`。
//!
//! cpp 各 `luau-*` CLI 的 `main` 由各自 `CLI/src/*.cpp` 提供；Rust 侧拆成
//! lib `run(args)` + bin 薄入口后，五个 bin 的 `main.rs` 逐字相同（仅 crate
//! 名不同），收口为此宏。
//!
//! `exit` 形（默认）：`run` 返回 `i32` 退出码，经 `std::process::exit` 上抛；
//! `unit` 形：`run` 自管退出（如 reduce 的 `help()` 内联 `exit(1)`），返回 `()`。

/// 见模块文档。调用形如 `ulua_cli_lib::cli_main!(ulua_ast_cli::run_main);`
/// 或 `ulua_cli_lib::cli_main!(unit: ulua_reduce_cli::run_main);`。
#[macro_export]
macro_rules! cli_main {
  ($run:path) => {
    fn main() {
      ::std::process::exit($run(&$crate::functions::argv::argv()));
    }
  };
  (unit: $run:path) => {
    fn main() {
      $run(&$crate::functions::argv::argv());
    }
  };
}
