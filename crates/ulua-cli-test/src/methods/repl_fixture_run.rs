use ulua_repl_cli::functions::run_code::run_code;

use crate::records::repl_fixture::ReplFixture;

impl ReplFixture {
  /// cpp 测试里 `runCode(L, src)` 的等价调用：在 fixture 的沙箱主线程上执行源码。
  ///
  /// 与 cpp 一样丢弃 `runCode` 返回的错误文本（错误经 `_PRETTYPRINT` 进
  /// `capturedoutput`，由断言检查）；接收者 `&mut self` 让借用检查器把这次 VM
  /// 写入与后续读输出的调用严格排他。
  pub fn run(&mut self, src: &str) {
    // `run_code` 已按 review.md §2 收编为 `&mut LuaState` 借用形的安全 fn：句柄经
    // `state_mut` 在夹具层唯一物化点交出，`src` 为合法 UTF-8 源码。
    let _ = run_code(self.state_mut(), src);
  }
}
