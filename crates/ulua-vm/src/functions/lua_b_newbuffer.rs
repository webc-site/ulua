use core::ptr::write_bytes;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::lua_m_newgco, lua_m_toobig::lua_m_toobig},
  macros::{lua_c_init::luaC_init, max_buffer_size::MAX_BUFFER_SIZE, sizebuffer::sizebuffer},
  records::{lua_state::LuaState, luau_buffer::LuauBuffer as Buffer},
};

/// 调用序契约（正确性，非内存安全——r12-w6 收形：`l` 的存活/独占已由 `&mut LuaState` 承载，
/// `l.activememcat` 为引用场域直读；体内被解引用的 `b` 是本函数经 `lua_m_newgco` 现取的新分配
/// GC 对象（判例 2「内部产生、不变量保护」），`lua_m_toobig`/`lua_m_newgco`/`luaC_init!` 的
/// 裸 `l` 转手各经一次 `l.as_mut_ptr()` 就地重建、借用窗止于当句（r16-v21 判例），故本体降为
/// 安全 `fn`）：`s > MAX_BUFFER_SIZE` 时 `lua_m_toobig` 以 longjmp 发散，GC 步进可重入——
/// 须在可捕获错误的受保护帧内调用；返回的 `b` 裸指针存活前提由 luaM 分配契约与本帧承载。
/// cpp `lbuffer.cpp:19`。
pub(crate) fn lua_b_newbuffer(l: &mut LuaState, s: usize) -> *mut Buffer {
  // SAFETY: 契约见文档——`l` 存活独占（引用形）、受保护帧（toobig/GC 重入）；`b` 为
  // `lua_m_newgco` 按 `sizebuffer(s)` 现配的存活对象，`len` 字段写与 `write_bytes` 清零
  // 均落在该数据界内。
  unsafe {
    if s > MAX_BUFFER_SIZE as usize {
      lua_m_toobig(l.as_mut_ptr());
    }

    let b = lua_m_newgco(l.as_mut_ptr(), sizebuffer(s), l.activememcat) as *mut Buffer;
    luaC_init!(l.as_mut_ptr(), b, LuaType::Buffer as i32);
    (*b).len = s as u32;
    write_bytes((*b).data.as_mut_ptr(), 0, (*b).len as usize);
    b
  }
}
