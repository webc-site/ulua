use crate::records::repl_fixture::ReplFixture;

impl ReplFixture {
  /// cpp 测试里的 `lua_resume(L, nullptr, 0)`：以默认参数恢复 fixture 的沙箱线程。
  ///
  /// 接收者 `&mut self`：`lua_resume` 会推进协程栈，属于 VM 写入。
  pub fn resume(&mut self) {
    // 空 from（主线程首启）的 C 契约由 `LuaState::resume_main` 专用入口收口。
    self.state_mut().resume_main(0);
  }
}
