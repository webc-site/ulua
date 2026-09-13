use core::{ffi::c_int, mem::size_of, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::luaM_newgco_, lua_m_toobig::lua_m_toobig},
  macros::{lua_c_init::luaC_init, sizeudata::sizeudata},
  records::udata::Udata,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_u_newudata(l: *mut lua_State, s: usize, tag: c_int) -> *mut Udata {
  unsafe {
    if s > c_int::MAX as usize - size_of::<Udata>() {
      lua_m_toobig(l);
    }

    let u = luaM_newgco_(l, sizeudata(s), (*l).activememcat) as *mut Udata;
    luaC_init!(l, u, LuaType::UserData as c_int);
    (*u).len = s as c_int;
    (*u).metatable = null_mut();
    LUAU_ASSERT!((0..=255).contains(&tag));
    (*u).tag = tag as u8;
    u
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaU_newudata")]
pub unsafe extern "C-unwind" fn lua_u_newudata_export(
  l: *mut lua_State,
  s: usize,
  tag: c_int,
) -> *mut Udata {
  unsafe { lua_u_newudata(l, s, tag) }
}

pub use lua_u_newudata as luaU_newudata;
