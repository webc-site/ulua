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
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_luaL_checkoption"))]
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

    // 选项表为 NUL 结尾指针数组：CStr 比较即字节序列比较，等价 cpp `strcmp == 0`
    let name_cstr = CStr::from_ptr(name);
    let mut i: i32 = 0;
    loop {
      let opt = *lst.add(i as usize);
      if opt.is_null() {
        break;
      }
      if CStr::from_ptr(opt) == name_cstr {
        return i;
      }
      i += 1;
    }

    let msg = lua_pushfstring_l(
      l,
      c"invalid option '%s'".as_ptr(),
      format_args!("invalid option '{}'", name_cstr.to_string_lossy()),
    );
    let msg_str = CStr::from_ptr(msg).to_string_lossy();
    lua_l_argerror_l(l, narg, msg_str.as_ref())
  }
}
