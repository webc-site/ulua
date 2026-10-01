//! 进程内退出码测试：`run_main(&args)` 接收注入的参数，无需 spawn 子进程
//! （对齐 `ulua-reduce-cli/tests/reduce_roundtrip.rs` 的用法）。
//!
//! 行为对照 `CLI/src/Ast.cpp`：`--help` → 0；缺少操作数或源文件读不到 → 1。

use std::{env::temp_dir, fs, process};

use ulua_ast_cli::run_main;

/// 组装 argv：首元素是程序名，其余由用例给出
fn argv(rest: &[&str]) -> Vec<String> {
  ["ulua-ast"]
    .iter()
    .chain(rest)
    .map(|arg| (*arg).to_string())
    .collect()
}

#[test]
fn help_exits_zero() {
  assert_eq!(run_main(&argv(&["--help"])), 0);
}

#[test]
fn missing_operand_exits_one() {
  assert_eq!(run_main(&argv(&[])), 1);
}

#[test]
fn unreadable_source_exits_one() {
  // 临时目录下一个必然不存在的路径：等价 cpp `readFile` 返回 nullopt
  let missing = temp_dir().join(format!("ulua-ast-cli-missing-{}.luau", process::id()));
  let _ = fs::remove_file(&missing);
  assert_eq!(run_main(&argv(&[missing.to_string_lossy().as_ref()])), 1);
}
