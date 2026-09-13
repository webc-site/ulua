use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_m_freegco::luaM_freegco_, unlinkstr::unlinkstr},
  macros::sizestring::sizestring,
  records::{gc_object::GCObject, lua_page::lua_Page, t_string::tstring},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_s_free(l: *mut lua_State, ts: *mut tstring, page: *mut lua_Page) {
  unsafe {
    if unlinkstr(l, ts) {
      (*(*l).global).strt.nuse = (*(*l).global).strt.nuse.wrapping_sub(1);
    } else {
      LUAU_ASSERT!((*ts).next.is_null());
    }

    luaM_freegco_(
      l,
      ts as *mut GCObject,
      sizestring((*ts).len as usize),
      (*ts).hdr.memcat,
      page,
    );
  }
}

pub use lua_s_free as luaS_free;
