use core::ptr::null_mut;

use ulua_vm::functions::lua_resume::lua_resume;

use crate::records::repl_fixture::ReplFixture;

impl ReplFixture {
  /// cpp 测试里的 `lua_resume(L, nullptr, 0)`：以默认参数恢复 fixture 的沙箱线程。
  ///
  /// 接收者 `&mut self`：`lua_resume` 会推进协程栈，属于 VM 写入。
  pub fn resume(&mut self) {
    // Safety: `self.l()` 指向初始化完成的主线程，首次 resume 传入空的 from 状态合法。
    unsafe { lua_resume(self.l(), null_mut(), 0) };
  }
}
