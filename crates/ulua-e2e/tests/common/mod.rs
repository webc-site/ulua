//! ulua 端到端集成测试的公共辅助：各 `tests/*.rs` 以 `mod common;` 引入。
//! 并非每个文件都用满全部辅助，但导出面按 `bin` / `write_script` / `cli_case!`
//! 三处实际消费收敛。
//! 「缺失文件」用例专用夹具见同级 `missing_paths.rs`（仅由消费它的文件单独 `#[path]`
//! 引入，避免破坏本文件的消费收敛纪律）。

use std::{
  env::{consts::EXE_SUFFIX, current_exe, var_os},
  fs::write,
  path::PathBuf,
};

use assert_cmd::Command;
use tempfile::TempDir;

/// 单命令 CLI 断言夹具：`cli_case!(<至 assert 前的 Command 链> => success|failure
/// [, stdout|stderr(谓词)]*)`，展开为 `<链>.assert().<状态>()[.<流>(<谓词>)]*`。
///
/// 展开式是表达式（求值为 `&mut Assert`），可用 `let` 绑定后续加断言或取 output。
/// 收敛六个二进制的 help / unknown-flag / 缺失路径 / 临时脚本运行等同形流水线用例；
/// 谓词（含 `.and` / `.or` / `.not` 组合、`predicate::function`、`stdout("")`
/// 这类 `IntoOutputPredicate` 形式）在调用点原样书写，经 `$pred:expr` 透传。
macro_rules! cli_case {
  ( $cmd:expr => $status:ident $(, $stream:ident ( $pred:expr $(,)? ) )* ) => {
    $cmd.assert().$status() $( .$stream($pred) )*
  };
}

/// 定位六个发行 CLI 二进制之一，包成 `assert_cmd::Command`。
///
/// `assert_cmd::Command::cargo_bin` 只能找到*当前* crate 的二进制（它读
/// `CARGO_BIN_EXE_<name>`，cargo 只为被测包设置）。本仓的二进制分布在同级 crate，
/// 故先从测试自身可执行文件位置回溯到活动 profile 目录，再退回
/// `CARGO_TARGET_DIR` / `<workspace 根>/target`。
pub fn bin(name: &str) -> Command {
  let mut candidates: Vec<PathBuf> = Vec::new();

  // 带平台可执行后缀：Unix 下是 `ulua-analyze`，Windows 下是 `ulua-analyze.exe`，
  // 直接拼裸名在 Windows 永远 `.exists()` 为假，spawn 类用例会全部找不到二进制。
  let file_name = format!("{name}{EXE_SUFFIX}");

  // 最可靠的来源：本测试可执行文件自身的位置。cargo / nextest 都从
  // `<target>/<profile>/deps/<test-exe>` 运行集成测试，故回溯到活动 profile 目录
  //（`debug` 或 `release`）即可拿到 workspace `[[bin]]` 产物，即便全局设了
  // CARGO_TARGET_DIR 也成立。
  if let Ok(exe) = current_exe() {
    let mut cur = exe.parent();
    while let Some(dir) = cur {
      if dir
        .file_name()
        .is_some_and(|name| name == "debug" || name == "release")
      {
        candidates.push(dir.join(&file_name));
        break;
      }
      cur = dir.parent();
    }
  }

  // 兜底：显式 `CARGO_TARGET_DIR`，再退回 `<workspace 根>/target`。
  let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  // crates/ulua-e2e -> crates -> <workspace 根>
  let workspace_target = manifest_dir
    .parent()
    .and_then(|p| p.parent())
    .map(|root| root.join("target"));
  let target_roots: Vec<PathBuf> = var_os("CARGO_TARGET_DIR")
    .map(PathBuf::from)
    .into_iter()
    .chain(workspace_target)
    .collect();
  for target in &target_roots {
    candidates.push(target.join("debug").join(&file_name));
    candidates.push(target.join("release").join(&file_name));
  }

  // 找不到二进制属于环境未就绪（未 build workspace bins），此处失败即中止用例，
  // 无任何可恢复语义，故用 panic 明确报告查找路径。
  let path = candidates.iter().find(|p| p.exists()).unwrap_or_else(|| {
    panic!(
      "could not locate binary {name}; looked in {:?}. \
             Build the workspace bins first (cargo build --workspace --bins).",
      candidates
    )
  });
  Command::new(path)
}

/// 新建临时目录并把 `source` 写入 `<dir>/<name>`，返回该目录（须保活，否则文件被删）
/// 与完整路径。
pub fn write_script(name: &str, source: &str) -> (TempDir, PathBuf) {
  let dir = tempfile::tempdir().expect("create tempdir");
  let path = dir.path().join(name);
  // 单次 `fs::write` 完成建文件 + 落盘，免 `File` + `write_all` + `flush` 三段样板。
  write(&path, source).expect("write script");
  (dir, path)
}
