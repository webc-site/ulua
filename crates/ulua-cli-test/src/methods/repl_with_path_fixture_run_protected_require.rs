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

    // `run_code` 已按 review.md §2 收编为 `&mut LuaState` 借用形的安全 fn：句柄经
    // `state_mut` 在夹具层唯一物化点交出，`code` 为合法源码。
    let _ = run_code(self.state_mut(), &code);
  }
}
