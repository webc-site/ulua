use core::{
  ffi::{c_int, c_void},
  mem::size_of,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_newgco::luaM_newgco_,
  macros::{isblack::isblack, isdead::isdead, lua_c_init::luaC_init, upisopen::upisopen},
  records::{gc_object::GCObject, up_val::UpVal},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_f_findupval(l: *mut lua_State, level: StkId) -> *mut UpVal {
  unsafe {
    let g = (*l).global;
    let mut pp: *mut *mut UpVal = core::ptr::addr_of_mut!((*l).openupval);

    while !(*pp).is_null() && (*(*pp)).v >= level {
      let p = *pp;
      LUAU_ASSERT!(!isdead!(g, p as *mut GCObject));
      LUAU_ASSERT!(upisopen!(p));
      if (*p).v == level {
        return p;
      }

      pp = core::ptr::addr_of_mut!((*p).u.open.threadnext);
    }

    LUAU_ASSERT!((*l).isactive);
    LUAU_ASSERT!(!isblack!(l as *mut GCObject));

    let uv = luaM_newgco_(l, size_of::<UpVal>(), (*l).activememcat) as *mut UpVal;

    luaC_init!(l, uv, LuaType::Upval as c_int);
    (*uv).markedopen = 0;
    (*uv).v = level;

    (*uv).u.open.threadnext = *pp;
    *pp = uv;

    let uvhead = core::ptr::addr_of_mut!((*g).uvhead);
    (*uv).u.open.prev = uvhead;
    (*uv).u.open.next = (*g).uvhead.u.open.next;
    (*(*uv).u.open.next).u.open.prev = uv;
    (*g).uvhead.u.open.next = uv;

    LUAU_ASSERT!((*(*uv).u.open.next).u.open.prev == uv && (*(*uv).u.open.prev).u.open.next == uv);

    uv
  }
}

pub use lua_f_findupval as luaF_findupval;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaF_findupval")]
pub unsafe extern "C-unwind" fn lua_f_findupval_export(
  l: *mut lua_State,
  level: StkId,
) -> *mut c_void {
  unsafe { lua_f_findupval(l, level).cast() }
}
