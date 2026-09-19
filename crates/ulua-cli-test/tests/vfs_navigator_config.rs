//! `VfsNavigator` 配置文件状态测试（review A1 C3：上游只认 `.luaurc` 与
//! `.config.luau`，本项目不再发明 legacy `.uluarc` 分支）。

use std::{fs::File, io::Write, path::Path};

use tempfile::TempDir;
use ulua_cli_lib::{
  enums::{config_status::ConfigStatus, navigation_status::NavigationStatus},
  records::vfs_navigator::VfsNavigator,
};

/// 让导航器指向 `dir` 目录模块
///
/// cpp `getConfigPath`（VfsNavigator.cpp:222-240）是 `realPath` 去掉模块后缀后
/// 再拼配置文件名，故目录模块 `dir` 的配置就在 `dir/.luaurc`。
fn navigator_at_module_dir(dir: &Path) -> VfsNavigator {
  let mut navigator = VfsNavigator::default();
  let status = navigator.reset_to_path(&dir.to_string_lossy());
  assert_eq!(status, NavigationStatus::Success);
  navigator
}

/// 配置文件存在性矩阵；`.uluarc` 单独存在时必须视为无配置
#[test]
fn config_status_matrix() {
  let cases: &[(&[&str], ConfigStatus)] = &[
    (&[], ConfigStatus::Absent),
    (&[".uluarc"], ConfigStatus::Absent),
    (&[".uluarc", ".config.luau"], ConfigStatus::PresentLuau),
    (&[".luaurc"], ConfigStatus::PresentJson),
    (&[".config.luau"], ConfigStatus::PresentLuau),
    (&[".luaurc", ".config.luau"], ConfigStatus::Ambiguous),
    (&[".luaurc", ".uluarc"], ConfigStatus::PresentJson),
  ];

  for (files, expected) in cases {
    let dir = TempDir::new().expect("创建临时目录失败");
    for name in *files {
      File::create(dir.path().join(name)).expect("创建配置文件失败");
    }

    let navigator = navigator_at_module_dir(dir.path());
    assert_eq!(&navigator.get_config_status(), expected, "夹具 {files:?}");
  }
}

/// `get_config` 直读命中的配置文件，不回退到 `.uluarc`
#[test]
fn get_config_reads_only_upstream_names() {
  let dir = TempDir::new().expect("创建临时目录失败");
  File::create(dir.path().join(".uluarc"))
    .and_then(|mut f| f.write_all(b"{\"legacy\":true}"))
    .expect("写 .uluarc 失败");
  let navigator = navigator_at_module_dir(dir.path());
  assert_eq!(navigator.get_config_status(), ConfigStatus::Absent);

  File::create(dir.path().join(".luaurc"))
    .and_then(|mut f| f.write_all(b"{\"names\":\"json\"}"))
    .expect("写 .luaurc 失败");
  assert_eq!(
    navigator.get_config().as_deref(),
    Some("{\"names\":\"json\"}"),
    "PresentJson 必须读 .luaurc"
  );

  let luau_dir = TempDir::new().expect("创建临时目录失败");
  File::create(luau_dir.path().join(".config.luau"))
    .and_then(|mut f| f.write_all(b"return { names = \"luau\" }"))
    .expect("写 .config.luau 失败");
  let luau_navigator = navigator_at_module_dir(luau_dir.path());
  assert_eq!(
    luau_navigator.get_config().as_deref(),
    Some("return { names = \"luau\" }"),
    "PresentLuau 必须读 .config.luau"
  );
}
