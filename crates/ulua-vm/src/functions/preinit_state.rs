use core::ptr;

use crate::records::{global_state::global_State, lua_state::LuaState};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn preinit_state(l: *mut LuaState, g: *mut global_State) {
  unsafe {
    // 头三字段（tt/marked/memcat）由 `lua_newstate` 先行写入，此处快照保留；其余
    // 字段一次类型化全零写入取代原 17 个 null_mut()/0 逐字段写。全零覆盖面比原写集
    // 多出 top/base/stack 等字段——它们在 cpp 里为分配野值、由后续 `stack_init` /
    // `open_base_ci` 全部覆写后才可达，cpp `LuauEasyStateInit` 快路径的
    // `memset(l, 0, sizeof(LG))` 同款形态，无观测差异。
    let hdr = (*l).hdr;
    ptr::write(
      l,
      LuaState {
        hdr,
        global: g,
        ..Default::default()
      },
    );
  }
}
