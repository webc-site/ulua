use core::{ffi::c_char, ptr::eq};

use crate::{
  functions::{cstr_bytes, index_2_addr::index_2_addr, lua_v_settable::lua_v_settable},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_o_nilobject::LUA_O_NILOBJECT,
    lua_s_new::lua_s_new, setsvalue::setsvalue,
  },
  records::{lua_state::LuaState, lua_t_value::TValue},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setfield_bytes(l: *mut LuaState, idx: i32, k: &[u8]) {
  unsafe {
    api_checknelems!(l, 1);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(t, LUA_O_NILOBJECT));

    let mut key = TValue::default();
    setsvalue!(l, &mut key, lua_s_new(l, k));
    lua_v_settable(l, t, &key, (*l).top.sub(1));
    (*l).top = (*l).top.sub(1);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setfield(l: *mut LuaState, idx: i32, k: *const c_char) {
  unsafe { lua_setfield_bytes(l, idx, cstr_bytes(k)) }
}
