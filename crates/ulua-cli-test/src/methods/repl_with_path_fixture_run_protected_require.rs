use ulua_repl_cli::functions::run_code::run_code;

use crate::records::repl_with_path_fixture::ReplWithPathFixture;

/// cpp `ReplWithPathFixture::runProtectedRequire`：在 fixture 主线程上以
/// `pcall` 包裹 `require`，错误只进 `capturedoutput` 而不中断测试。
pub fn repl_with_path_fixture_run_protected_require(fixture: &ReplWithPathFixture, path: &str) {
  let code = format!("return pcall(function() return require(\"{}\") end)", path);

  // SAFETY: `fixture.l` 为 fixture 持有的活跃主线程状态；`code` 为合法源码。
  let _ = unsafe { run_code(fixture.l, &code) };
}
