use crate::{
  functions::lua_m_freegco::lua_m_freegco,
  macros::{obj_2_gco::obj2gco, sizebuffer::sizebuffer},
  records::{lua_page::lua_Page, lua_state::LuaState, luau_buffer::LuauBuffer as Buffer},
};

/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v29 收形），但仍经 `l.as_mut_ptr()` 一次就地重建
/// 转手给仍收裸形的 `lua_m_freegco`（其经 global 记账并回调 `frealloc`），屏障按 r16-v21 判例保留；
/// `b` 须为待释放的存活 `LuauBuffer`，读其 `len/memcat`；`page` 为持有该 buffer 的 `lua_Page`，须与分配时一致。
/// cpp `lbuffer.cpp:26`。
pub(crate) unsafe fn lua_b_freebuffer(l: &mut LuaState, b: *mut Buffer, page: *mut lua_Page) {
  unsafe {
    lua_m_freegco(
      l.as_mut_ptr(),
      obj2gco!(b),
      sizebuffer((*b).len as usize),
      (*b).memcat,
      page,
    );
  }
}
