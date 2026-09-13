use core::ffi::c_char;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_s_newlstr::luaS_newlstr, luai_num_2_str::luai_num2str},
  macros::{
    luai_maxnum_2_str::LUAI_MAXNUM2STR, nvalue::nvalue, setsvalue::setsvalue,
    ttisnumber::ttisnumber,
  },
  type_aliases::{lua_state::LuaState, stk_id::StkId},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_v_tostring(l: *mut LuaState, obj: StkId) -> i32 {
  unsafe {
    if !ttisnumber!(obj) {
      0
    } else {
      let mut s = [0 as c_char; LUAI_MAXNUM2STR as usize];
      let n = nvalue!(obj);
      let e = luai_num2str(s.as_mut_ptr(), n);
      LUAU_ASSERT!((e as usize) < (s.as_ptr() as usize + s.len()));
      setsvalue!(
        l,
        obj,
        luaS_newlstr(l, s.as_ptr(), e.offset_from(s.as_ptr()) as usize)
      );
      1
    }
  }
}
