//! `tests/file_utils.rs` / `tests/vfs_navigator_config.rs` 共用的临时目录样板：
//! `TempDir::new()` + 统一失败文案单点收口。仅由这两个测试文件以
//! `#[path = "common/temp_dir.rs"] mod temp_dir_support;` 引入，避免污染
//! `common/mod.rs` 的「每个声明者都用满全部导出」纪律（不设 `#[allow(dead_code)]`）。

use tempfile::TempDir;

/// 新建唯一临时目录，失败即中止用例。
pub fn temp_dir() -> TempDir {
  TempDir::new().expect("创建临时目录失败")
}
