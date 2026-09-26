use alloc::string::String;

use ulua_cli_lib::functions::{
  get_current_working_directory::get_current_working_directory, get_parent_path::get_parent_path,
  is_directory::is_directory,
};

use crate::{enums::path_type::PathType, records::repl_with_path_fixture::ReplWithPathFixture};

/// 向上遍历祖先目录查找 Luau checkout 的最大深度（cpp 硬编码 20）
const MAX_ANCESTRY_WALK_DEPTH: usize = 20;

/// cpp `REQUIRE_MESSAGE(..., "Error getting Luau path")` 的失败文本。
const NO_LUAU_PATH: &str = "Error getting Luau path";

impl ReplWithPathFixture {
  /// cpp `ReplWithPathFixture::getLuauDirectory`：cpp 里是成员函数，但只读文件
  /// 系统、不触碰 fixture 状态，故 Rust 侧收为关联函数（不必先建 VM 就能算路径）。
  pub fn get_luau_directory(type_: PathType) -> String {
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
    let cwd = cwd.expect(NO_LUAU_PATH);
    let cwd_normalized = cwd.replace('\\', "/");
    let mut luau_dir_abs = cwd_normalized;

    // cpp 同款「有界重试」：从 cwd 起逐层上溯最多 20 个祖先目录，每轮同时推进
    // 绝对路径（`get_parent_path`）与相对路径（`..` 串联）两份字符串——不是对
    // 集合下标的遍历（无 `v.len()` 可迭代），故保留计数形态并以常量命名上界。
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
      let parent_path = parent_path.expect(NO_LUAU_PATH);
      luau_dir_abs = parent_path;
    }

    panic!("{NO_LUAU_PATH}");
  }
}
