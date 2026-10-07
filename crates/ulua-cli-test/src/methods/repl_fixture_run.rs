use ulua_repl_cli::functions::run_code::run_code;

use crate::records::repl_fixture::ReplFixture;

impl ReplFixture {
  /// cpp 测试里 `runCode(L, src)` 的等价调用：在 fixture 的沙箱主线程上执行源码。
  ///
  /// 与 cpp 一样丢弃 `runCode` 返回的错误文本（错误经 `_PRETTYPRINT` 进
  /// `capturedoutput`，由断言检查）；接收者 `&mut self` 让借用检查器把这次 VM
  /// 写入与后续读输出的调用严格排他。
  pub fn run(&mut self, src: &str) {
    // Safety: `self.l()` 指向 `ReplFixture::new` 初始化的合法 lua_State，`src` 为合法 UTF-8 源码。
    let _ = unsafe { run_code(self.l(), src) };
  }
}
