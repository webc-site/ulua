use crate::records::lua_state::LuaState;

#[inline]
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`（读其 `stacksize` 字段计算扩容后的目标大小）。
pub(crate) unsafe fn getgrownstacksize(l: *mut LuaState, n: i32) -> i32 {
  // Safety: 契约保证 `l` 指向存活 LuaState，此处仅读 stacksize 字段
  unsafe {
    if n <= (*l).stacksize {
      2 * (*l).stacksize
    } else {
      (*l).stacksize + n
    }
  }
}
