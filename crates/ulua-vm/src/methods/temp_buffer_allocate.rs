use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::lua_m_newarray::luaM_newarray,
  records::{lua_state::LuaState, temp_buffer::TempBuffer},
};

impl<T> TempBuffer<T> {
  /// # Safety
  ///
  /// `self` 必须为尚未持有分配的 `TempBuffer`（`l` 为 null，入口 `LUAU_ASSERT` 兜底），
  /// 否则会覆盖并泄漏旧 `data`；`l` 必须指向存活的 `LuaState`，`luaM_newarray!` 可能
  /// 触发 GC 或在 OOM 时经错误处理抛出，调用方需处于可接受抛出的上下文。
  pub(crate) unsafe fn allocate(&mut self, l: *mut LuaState, count: usize) {
    // Safety: 契约保证 self 未持有分配、l 存活——luaM_newarray 随 l 分配 count 个 T 并可能触发 GC
    unsafe {
      LUAU_ASSERT!(self.l.is_null());
      self.l = l;
      self.data = luaM_newarray!(l, count, T, 0);
      self.count = count;
    }
  }
}
