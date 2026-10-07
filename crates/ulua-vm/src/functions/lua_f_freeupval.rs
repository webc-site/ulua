use core::mem::size_of;

use crate::{
  functions::lua_m_freegco::lua_m_freegco,
  records::{gc_object::GcObject, lua_page::lua_Page, lua_state::LuaState, up_val::UpVal},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_f_freeupval(l: *mut LuaState, uv: *mut UpVal, page: *mut lua_Page) {
  unsafe {
    lua_m_freegco(
      l,
      uv as *mut GcObject,
      size_of::<UpVal>(),
      (*uv).hdr.memcat,
      page,
    );
  }
}
