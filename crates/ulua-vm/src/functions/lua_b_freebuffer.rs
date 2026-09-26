use crate::{
  functions::lua_m_freegco::lua_m_freegco,
  macros::{obj_2_gco::obj2gco, sizebuffer::sizebuffer},
  records::{lua_page::lua_Page, lua_state::LuaState, luau_buffer::LuauBuffer as Buffer},
};

/// # Safety
/// `l` 须为存活 `LuaState`（`lua_m_freegco` 经其 `global` 记账并回调 `frealloc`）；`b` 须为待释放的
/// 存活 `LuauBuffer`，读其 `len/memcat`；`page` 为持有该 buffer 的 `lua_Page`，须与分配时一致。
/// cpp `lbuffer.cpp:26`。
pub(crate) unsafe fn lua_b_freebuffer(l: *mut LuaState, b: *mut Buffer, page: *mut lua_Page) {
  unsafe {
    lua_m_freegco(
      l,
      obj2gco!(b),
      sizebuffer((*b).len as usize),
      (*b).memcat,
      page,
    );
  }
}
