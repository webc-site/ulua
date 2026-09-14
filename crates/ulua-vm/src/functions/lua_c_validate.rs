use core::{ffi::c_void, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_m_visitgco::lua_m_visitgco, validategco::validategco, validategraylist::validategraylist,
  },
  macros::{checkliveness::checkliveness, isblack::isblack, isdead::isdead, upisopen::upisopen},
  records::{
    gc_object::GCObject, global_state::global_State, lua_page::lua_Page, lua_state::lua_State,
  },
  type_aliases::up_val::UpVal,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_validate(l: *mut lua_State) {
  unsafe {
    let g: *mut global_State = (*l).global;

    // The obj2gco! macro relies on ttype!, which expects a .tt() method.
    // Since the Rust records for lua_State, LuaTable, and UpVal use a field hdr.tt instead of a method,
    // we must cast these pointers to *mut GCObject (which is what obj2gco! effectively does via cast_to!)
    // to bypass the ttype! check on the specific record types.
    LUAU_ASSERT!(!isdead!(g, (*g).mainthread as *mut GCObject));
    checkliveness!(g, &(*g).registry);

    for &mt in (*g).mt.iter() {
      if !mt.is_null() {
        LUAU_ASSERT!(!isdead!(g, mt as *mut GCObject));
      }
    }

    validategraylist(g, (*g).weak as *mut GCObject);
    validategraylist(g, (*g).gray as *mut GCObject);
    validategraylist(g, (*g).grayagain as *mut GCObject);

    validategco(
      l as *mut c_void,
      null_mut::<lua_Page>(),
      (*g).mainthread as *mut GCObject,
    );

    lua_m_visitgco(l, l as *mut c_void, validategco as *mut c_void);

    let mut uv: *mut UpVal = (*g).uvhead.u.open.next;
    while uv != &mut (*g).uvhead {
      LUAU_ASSERT!((*uv).hdr.tt == LuaType::Upval as u8);
      LUAU_ASSERT!(upisopen!(uv));
      LUAU_ASSERT!(
        (*(*uv).u.open.next).u.open.prev == uv && (*(*uv).u.open.prev).u.open.next == uv
      );
      // open upvalues are never black
      LUAU_ASSERT!(!isblack!(uv as *mut GCObject));
      uv = (*uv).u.open.next;
    }
  }
}
