//! `ulua-cli-lib` 文件工具（`traverse_directory` / `get_source_files_from_slice`）测试。
//!
//! 覆盖 review A1 C2：cpp `traverseDirectoryRec`（FileUtils.cpp:260,318）丢弃递归
//! 返回值，子目录遍历失败只跳过该子树，不得中断同层/父级收集。

use std::{
  collections::BTreeSet,
  fs::{File, create_dir_all},
  path::Path,
};
#[cfg(unix)]
use std::{
  fs::{Permissions, create_dir, read_dir, set_permissions},
  os::unix::fs::PermissionsExt,
};

use tempfile::TempDir;
use ulua_cli_lib::functions::{
  get_source_files::get_source_files_from_slice, normalize_path::normalize_path,
  traverse_directory::traverse_directory,
};

/// 建空文件（连同父目录）
fn touch(path: &Path) {
  if let Some(parent) = path.parent() {
    create_dir_all(parent).expect("创建父目录失败");
  }
  File::create(path).expect("创建文件失败");
}

/// 递归收集 `root` 下全部条目，返回 `traverse_directory` 的返回值与收集结果
fn collect(root: &Path) -> (bool, BTreeSet<String>) {
  let mut files = BTreeSet::new();
  let ok = traverse_directory(&root.to_string_lossy(), |name| {
    files.insert(name.to_string());
  });
  (ok, files)
}

/// 把 `path` 置为不可读；返回是否真的读不了（以 root 跑测试时 0o000 仍可读写）
#[cfg(unix)]
fn make_unreadable(path: &Path) -> bool {
  set_permissions(path, Permissions::from_mode(0o000)).expect("改权限失败");
  read_dir(path).is_err()
}

/// 顶层目录不可读：返回 false 且无收集（镜像 cpp 只在 `read_dir` 失败处返回 false）
#[test]
#[cfg(unix)]
fn unreadable_root_dir_returns_false() {
  let dir = TempDir::new().expect("创建临时目录失败");
  let root = dir.path().join("tree");
  create_dir(&root).expect("创建目录失败");
  touch(&root.join("a.luau"));

  if !make_unreadable(&root) {
    set_permissions(&root, Permissions::from_mode(0o755)).expect("还原权限失败");
    return;
  }

  let (ok, files) = collect(&root);
  set_permissions(&root, Permissions::from_mode(0o755)).expect("还原权限失败");

  assert!(!ok);
  assert!(files.is_empty());
}

/// 子目录不可读：整体仍返回 true，同层与后续文件全部收集（C2 回归）
#[test]
#[cfg(unix)]
fn unreadable_subdir_does_not_abort_sibling_scan() {
  let dir = TempDir::new().expect("创建临时目录失败");
  let root = dir.path();
  let blocked = root.join("blocked");
  create_dir(&blocked).expect("创建 blocked 失败");
  touch(&root.join("after.luau"));
  touch(&root.join("nested/deep.luau"));

  if !make_unreadable(&blocked) {
    set_permissions(&blocked, Permissions::from_mode(0o755)).expect("还原权限失败");
    return;
  }

  let (ok, files) = collect(root);
  set_permissions(&blocked, Permissions::from_mode(0o755)).expect("还原权限失败");

  let root_str = root.to_string_lossy();
  assert!(ok, "子目录不可读不得让整棵树遍历失败");
  assert_eq!(
    files,
    BTreeSet::from([
      format!("{root_str}/after.luau"),
      format!("{root_str}/nested/deep.luau"),
    ])
  );
}

/// 目录参数只收 `.lua`/`.luau`
#[test]
fn directory_argument_collects_only_lua_sources() {
  let dir = TempDir::new().expect("创建临时目录失败");
  let root = dir.path();
  touch(&root.join("keep.luau"));
  touch(&root.join("keep.lua"));
  touch(&root.join("skip.txt"));
  touch(&root.join("noext"));
  touch(&root.join("sub/deep.luau"));
  touch(&root.join("sub/deep.md"));

  let root_str = normalize_path(&root.to_string_lossy());
  let expected: BTreeSet<String> = BTreeSet::from([
    format!("{root_str}/keep.luau"),
    format!("{root_str}/keep.lua"),
    format!("{root_str}/sub/deep.luau"),
  ]);
  let files = get_source_files_from_slice(&["ulua".to_string(), root_str]);
  let got: BTreeSet<String> = files.into_iter().collect();

  assert_eq!(got, expected);
}

/// `--program-args` / `-a` 之后的参数不再作为源文件收集（早退）
#[test]
fn program_args_stops_source_file_collection() {
  let dir = TempDir::new().expect("创建临时目录失败");
  let root = dir.path();
  touch(&root.join("real.luau"));
  let root_str = normalize_path(&root.to_string_lossy());

  for flag in ["--program-args", "-a"] {
    let files = get_source_files_from_slice(&[
      "ulua".to_string(),
      root_str.clone(),
      flag.to_string(),
      "after.luau".to_string(),
    ]);
    assert_eq!(
      files,
      vec![format!("{root_str}/real.luau")],
      "{flag} 之后应停止收集"
    );
  }
}
