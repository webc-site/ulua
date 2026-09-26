use alloc::string::String;

use crate::{functions::captured_output::captured_output, records::repl_fixture::ReplFixture};

impl ReplFixture {
  /// cpp `ReplFixture::getCapturedOutput`；`ReplWithPathFixture` 通过类型别名共用。
  pub fn get_captured_output(&mut self) -> String {
    // Safety: `self.l()` 指向 fixture 初始化完成且定义了 `capturedoutput` 的状态
    unsafe { captured_output(self.l()) }
  }
}
