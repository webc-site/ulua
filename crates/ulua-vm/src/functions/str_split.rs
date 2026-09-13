use core::{
  ffi::{c_char, c_int},
  slice::from_raw_parts,
};

use crate::{
  functions::{
    lua_createtable::lua_createtable, lua_l_checklstring::lua_l_checklstring,
    lua_l_optlstring::lua_l_optlstring, lua_pushinteger::lua_pushinteger,
    lua_pushlstring::lua_pushlstring, lua_settable::lua_settable,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_str_split")]
pub(crate) unsafe extern "C-unwind" fn str_split(l: *mut lua_State) -> c_int {
  unsafe {
    let mut haystack_len: usize = 0;
    let haystack = lua_l_checklstring(l, 1, &mut haystack_len);
    let mut needle_len: usize = 0;
    let needle = lua_l_optlstring(l, 2, c",".as_ptr() as *const c_char, &mut needle_len);

    let begin = haystack;
    let end = haystack.add(haystack_len);
    let mut span_start = begin;
    let mut num_matches = 0;

    lua_createtable(l, 0, 0);

    let mut iter = begin;
    if needle_len == 0 {
      iter = iter.add(1);
    }

    // Don't iterate the last needleLen - 1 bytes of the string - they are
    // impossible to be splits and would let us compare past the end of the
    // buffer.
    let needle_bytes = from_raw_parts(needle as *const u8, needle_len);
    while iter <= end.offset(-(needle_len as isize)) {
      // u8 切片比较与 C memcmp 语义一致，且允许嵌入 null 字节
      let window = from_raw_parts(iter as *const u8, needle_len);
      if window == needle_bytes {
        num_matches += 1;
        lua_pushinteger(l, num_matches);
        lua_pushlstring(l, span_start, iter.offset_from(span_start) as usize);
        lua_settable(l, -3);

        span_start = iter.add(needle_len);
        if needle_len > 0 {
          iter = iter.add(needle_len - 1);
        }
      }
      iter = iter.add(1);
    }

    if needle_len > 0 {
      num_matches += 1;
      lua_pushinteger(l, num_matches);
      lua_pushlstring(l, span_start, end.offset_from(span_start) as usize);
      lua_settable(l, -3);
    }

    1
  }
}
