use alloc::string::String;

use ulua_cli_lib::functions::{
  get_current_working_directory::get_current_working_directory, get_parent_path::get_parent_path,
  is_directory::is_directory,
};

use crate::{enums::path_type::PathType, records::repl_with_path_fixture::ReplWithPathFixture};

/// 向上遍历祖先目录查找 Luau checkout 的最大深度（cpp 硬编码 20）
const MAX_ANCESTRY_WALK_DEPTH: usize = 20;

impl ReplWithPathFixture {
  /// cpp `ReplWithPathFixture::getLuauDirectory`：成员函数但不触碰 fixture 状态
  /// （只读文件系统），因此接收者仅作命名空间。
  pub fn get_luau_directory(&self, type_: PathType) -> String {
    // Require fixtures vendored alongside the crate (standalone / published-repo
    // layout). Checked before the cwd-walk that finds the upstream
    // luau/tests/require checkout in the original workspace. The require resolver
    // demands a `./`, `../` or `@` prefix, so return the vendored dir as a path
    // relative to CWD (CARGO_MANIFEST_DIR is at or under CWD for cargo/nextest,
    // so a "./..." form always exists).
    {
      // `CARGO_MANIFEST_DIR` uses the OS separator — backslashes on Windows —
      // so normalize to forward slashes. `cwd` below is normalized the same way;
      // without this the `strip_prefix` mismatched (backslash vs slash), the
      // function returned an absolute `D:\...` path, and the require resolver
      // (which requires a `./`/`../`/`@` prefix) rejected it — failing every
      // require-by-string test on Windows.
      let vendored_abs = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures").replace('\\', "/");
      let vendored_abs = vendored_abs.as_str();
      if is_directory(&format!("{}/tests/require", vendored_abs)) {
        match type_ {
          // Cache keys are built from the absolute form; keep it absolute.
          PathType::Absolute => return String::from(vendored_abs),
          // Require calls need a ./ prefix; the relative form resolves back
          // to vendored_abs against CWD, so the cache key still matches.
          PathType::Relative => {
            if let Some(cwd) = get_current_working_directory() {
              let cwd = cwd.replace('\\', "/");
              if vendored_abs == cwd {
                return String::from(".");
              }
              if let Some(rel) = vendored_abs.strip_prefix(&format!("{}/", cwd)) {
                return format!("./{}", rel);
              }
            }
            return String::from(vendored_abs);
          }
        }
      }
    }

    let mut luau_dir_rel = String::from(".");

    #[cfg(target_os = "ios")]
    {
      use crate::functions::get_resource_path::get_resource_path;
      let cwd0 = get_current_working_directory();
      let cwd = get_resource_path();
      if let (Some(res), Some(cwd_val)) = (cwd, cwd0) {
        if res.starts_with(&cwd_val) {
          luau_dir_rel = format!("./{}", &res[cwd_val.len()..]);
        }
      }
      // cpp:125-130：Xcode 之外跑 iOS 测试时用 TEST_SOURCE_ROOT 切回源码目录；
      // cpp 里重取的 cwd 由下方统一的 `get_current_working_directory()` 覆盖。
      if let Ok(repo_root) = std::env::var("TEST_SOURCE_ROOT")
        && std::env::set_current_dir(&repo_root).is_ok()
      {
        luau_dir_rel = String::from(".");
      }
    }

    let cwd = get_current_working_directory();
    let cwd = cwd.expect("Error getting Luau path");
    let cwd_normalized = cwd.replace('\\', "/");
    let mut luau_dir_abs = cwd_normalized;

    for _ in 0..MAX_ANCESTRY_WALK_DEPTH {
      let engine_test_dir = is_directory(&format!("{}/Client/Luau/tests", luau_dir_abs));
      let luau_test_dir = is_directory(&format!("{}/tests/require", luau_dir_abs));
      // In the workspace layout, the upstream fixtures live under a
      // nested `cpp/` (or `luau/`) checkout rather than directly at `<dir>/tests/require`.
      let cpp_subdir_test = is_directory(&format!("{}/cpp/tests/require", luau_dir_abs));
      let luau_subdir_test = is_directory(&format!("{}/luau/tests/require", luau_dir_abs));

      if engine_test_dir || luau_test_dir || cpp_subdir_test || luau_subdir_test {
        if engine_test_dir {
          luau_dir_rel = format!("{}/Client/Luau", luau_dir_rel);
          luau_dir_abs = format!("{}/Client/Luau", luau_dir_abs);
        } else if cpp_subdir_test {
          luau_dir_rel = format!("{}/cpp", luau_dir_rel);
          luau_dir_abs = format!("{}/cpp", luau_dir_abs);
        } else if luau_subdir_test {
          luau_dir_rel = format!("{}/luau", luau_dir_rel);
          luau_dir_abs = format!("{}/luau", luau_dir_abs);
        }

        match type_ {
          PathType::Relative => return luau_dir_rel,
          PathType::Absolute => return luau_dir_abs,
        }
      }

      if luau_dir_rel == "." {
        luau_dir_rel = "..".to_string();
      } else {
        luau_dir_rel = format!("{}/..", luau_dir_rel);
      }

      let parent_path = get_parent_path(&luau_dir_abs);
      let parent_path = parent_path.expect("Error getting Luau path");
      luau_dir_abs = parent_path;
    }

    panic!("Error getting Luau path");
  }
}
