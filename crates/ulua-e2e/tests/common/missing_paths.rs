//! 缺失路径用例的公共夹具：仅由需要「文件不存在」判据的测试文件以
//! `#[path = "common/missing_paths.rs"] mod missing_paths;` 引入，不并入
//! `common/mod.rs`——保持其中每个 item 被所有 `mod common;` 声明者消费的纪律
//! （不设 `#[allow(dead_code)]`，见 `ulua-vm/tests/common/mod.rs` 同型约定）。

use std::path::PathBuf;

use tempfile::TempDir;

/// 新建唯一临时目录并返回其下一个**不存在**的 `<dir>/<name>` 路径
/// （目录须与路径一起保活至断言完成）。
pub fn not_exist(name: &str) -> (TempDir, PathBuf) {
  let dir = tempfile::tempdir().expect("create tempdir");
  let path = dir.path().join(name);
  (dir, path)
}
