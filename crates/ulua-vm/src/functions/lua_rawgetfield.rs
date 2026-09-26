use core::ffi::c_char;

use crate::{
  functions::{
    cstr_bytes, ensure_stack::ensure_stack, index_2_addr::index_2_addr,
    lapi_barrier::lua_c_threadbarrier_lapi, lua_h_getstr::lua_h_getstr,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_s_new::lua_s_new, setobj_2_s::setobj_2_s,
    setsvalue::setsvalue, ttype::ttype,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawgetfield_bytes(l: *mut LuaState, idx: i32, k: &[u8]) -> i32 {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());

    let mut key = TValue::default();
    setsvalue!(l, &mut key, lua_s_new(l, k));
    setobj_2_s!(
      l,
      (*l).top,
      lua_h_getstr((*t).as_table_ptr(), key.as_string_ptr() as *mut _)
    );
    api_incr_top!(l);

    ttype!((*l).top.sub(1)) as i32
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_rawgetfield(l: *mut LuaState, idx: i32, k: *const c_char) -> i32 {
  unsafe { lua_rawgetfield_bytes(l, idx, cstr_bytes(k)) }
}
