use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
};

use crate::{
  enums::k_option::KOption,
  functions::{getdetails::getdetails, initheader::initheader, lua_pushinteger::lua_pushinteger},
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring},
  records::header::Header,
  type_aliases::lua_state::lua_State,
};

const MAXSSIZE: usize = 1 << 30;

/// # Safety
///
/// `l` must point to a valid, properly initialized `lua_State`.
pub(crate) unsafe extern "C-unwind" fn str_packsize(l: *mut lua_State) -> c_int {
  unsafe {
    let mut h = Header {
      l: null_mut(),
      islittle: 0,
      maxalign: 0,
    };
    let fmt_ptr = luaL_checkstring!(l, 1);
    let mut fmt = fmt_ptr;
    let mut totalsize: usize = 0;

    initheader(l, &mut h);

    while *fmt != b'\0' as c_char {
      let mut size: i32 = 0;
      let mut ntoalign: i32 = 0;
      let mut fmt_cursor = fmt;

      let opt = getdetails(&mut h, totalsize, &mut fmt_cursor, &mut size, &mut ntoalign);
      fmt = fmt_cursor;

      luaL_argcheck!(
        l,
        opt != KOption::Kstring && opt != KOption::Kzstr,
        1,
        "variable-length format"
      );

      let total_option_size = (size + ntoalign) as usize;
      luaL_argcheck!(
        l,
        totalsize <= MAXSSIZE - total_option_size,
        1,
        "format result too large"
      );

      totalsize += total_option_size;
    }

    lua_pushinteger(l, totalsize as c_int);
    1
  }
}
