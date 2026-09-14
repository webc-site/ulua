use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_f_closeupval::luaF_closeupval,
  macros::{isblack::isblack, isdead::isdead, upisopen::upisopen},
  records::{gc_object::GCObject, up_val::UpVal},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_f_close(l: *mut lua_State, level: StkId) {
  unsafe {
    let g = (*l).global;

    while !(*l).openupval.is_null() && (*(*l).openupval).v >= level {
      let uv: *mut UpVal = (*l).openupval;
      let o = uv as *mut GCObject;
      LUAU_ASSERT!(!isblack!(o) && upisopen!(uv));
      LUAU_ASSERT!(!isdead!(g, o));

      (*l).openupval = (*uv).u.open.threadnext;
      luaF_closeupval(l, uv, false);
    }
  }
}

pub use lua_f_close as luaF_close;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaF_close")]
pub unsafe extern "C-unwind" fn lua_f_close_export(l: *mut lua_State, level: StkId) {
  unsafe {
    lua_f_close(l, level);
  }
}
