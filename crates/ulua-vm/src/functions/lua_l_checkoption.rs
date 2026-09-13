use core::{
  ffi::{CStr, c_char, c_int},
  ptr::null_mut,
};

use crate::{
  functions::{
    lua_l_argerror_l::lua_l_argerror_l, lua_l_checklstring::lua_l_checklstring,
    lua_l_optlstring::lua_l_optlstring, lua_pushfstring_l::lua_pushfstring_l,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` must point to a valid `lua_State`.
/// `lst` must be a valid null-terminated array of null-terminated C strings.
/// `def` may be null or point to a valid null-terminated C string.
#[unsafe(export_name = "ulua_luaL_checkoption")]
pub unsafe fn lua_l_checkoption(
  l: *mut lua_State,
  narg: c_int,
  def: *const c_char,
  lst: *const *const c_char,
) -> c_int {
  unsafe {
    let name: *const c_char = if !def.is_null() {
      lua_l_optlstring(l, narg, def, null_mut())
    } else {
      lua_l_checklstring(l, narg, null_mut())
    };

    let mut i: i32 = 0;
    while !(*lst.add(i as usize)).is_null() {
      let opt = *lst.add(i as usize);
      if libc_strcmp(opt, name) == 0 {
        return i;
      }
      i += 1;
    }

    let name_str = CStr::from_ptr(name).to_string_lossy();
    let msg = lua_pushfstring_l(
      l,
      c"invalid option '%s'".as_ptr(),
      format_args!("invalid option '{}'", name_str),
    );
    let msg_str = CStr::from_ptr(msg).to_string_lossy();
    lua_l_argerror_l(l, narg, msg_str.as_ref())
  }
}

unsafe fn libc_strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
  unsafe {
    let mut i = 0;
    loop {
      let c1 = *s1.add(i) as u8;
      let c2 = *s2.add(i) as u8;
      if c1 != c2 {
        return if c1 < c2 { -1 } else { 1 };
      }
      if c1 == 0 {
        return 0;
      }
      i += 1;
    }
  }
}

pub use lua_l_checkoption as luaL_checkoption;
