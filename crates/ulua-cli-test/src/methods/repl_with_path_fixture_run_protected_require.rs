use ulua_repl_cli::functions::run_code::run_code;

use crate::records::repl_with_path_fixture::ReplWithPathFixture;

impl ReplWithPathFixture {
  /// cpp `ReplWithPathFixture::runProtectedRequire`：在 fixture 主线程上以
  /// `pcall` 包裹 `require`，错误只进 `capturedoutput` 而不中断测试。
  ///
  /// 接收者为 `&mut self`：本方法写入 VM，独占借用让借用检查器能把它与
  /// 读输出的调用（`get_captured_output` 等）严格排他。
  pub fn run_protected_require(&mut self, path: &str) {
    let code = format!("return pcall(function() return require(\"{path}\") end)");

    // Safety: `self.l()` 为 fixture 持有的活跃主线程状态；`code` 为合法源码。
    let _ = unsafe { run_code(self.l(), &code) };
  }
}
