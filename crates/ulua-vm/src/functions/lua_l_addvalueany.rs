//! Node: `cxx:Function:Luau.VM:VM/src/laux.cpp:529:luaL_addvalueany`
//! Source: `VM/src/laux.cpp:529-582` (hand-ported)

use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_addlstring::lua_l_addlstring, lua_l_addvalue::lua_l_addvalue,
    lua_l_tolstring::lua_l_tolstring, lua_toboolean::lua_toboolean,
    lua_tointeger_64::lua_tointeger_64, lua_tolstring::lua_tolstring, lua_tonumberx::lua_tonumberx,
    lua_type::lua_type, luai_int_2_str::luai_int2str, luai_num_2_str::luai_num2str,
  },
  macros::{luai_maxint_2_str::LUAI_MAXINT2STR, luai_maxnum_2_str::LUAI_MAXNUM2STR},
  records::lua_l_strbuf::LuaLStrbuf,
};

/// C++ `void luaL_addvalueany(luaL_Strbuf *b, int idx)` —
/// converts the value at stack index `idx` to its string representation
/// and appends it to buffer `b`.
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_l_addvalueany(b: *mut LuaLStrbuf, idx: c_int) {
  unsafe {
    let l = (*b).l;

    match lua_type(l, idx) {
      // cpp release 构建 LUAU_ASSERT 编译掉后 break 直落：不追加任何内容
      x if x == LuaType::None as c_int => {}
      x if x == LuaType::Nil as c_int => {
        lua_l_addlstring(b, c"nil".as_ptr(), 3);
      }
      x if x == LuaType::Boolean as c_int => {
        if lua_toboolean(l, idx) != 0 {
          lua_l_addlstring(b, c"true".as_ptr(), 4);
        } else {
          lua_l_addlstring(b, c"false".as_ptr(), 5);
        }
      }
      x if x == LuaType::Number as c_int => {
        let mut isnum: i32 = 0;
        let n = lua_tonumberx(l, idx, &mut isnum);
        let mut s = [0 as c_char; LUAI_MAXNUM2STR as usize];
        let e = luai_num2str(s.as_mut_ptr(), n);
        lua_l_addlstring(b, s.as_ptr(), e.offset_from(s.as_ptr()) as usize);
      }
      x if x == LuaType::String as c_int => {
        let mut len: usize = 0;
        let s = lua_tolstring(l, idx, &mut len);
        lua_l_addlstring(b, s, len);
      }
      x if x == LuaType::Integer as c_int => {
        let n = lua_tointeger_64(l, idx, null_mut());
        let mut s = [0 as c_char; LUAI_MAXINT2STR as usize];
        let e = luai_int2str(s.as_mut_ptr(), n);
        lua_l_addlstring(b, s.as_ptr(), e.offset_from(s.as_ptr()) as usize);
      }
      _ => {
        // note: luaL_addlstring assumes box is stored at top of stack, so we can't call it here
        // instead we use luaL_addvalue which will take the string from the top of the stack and add that
        let mut len: usize = 0;
        lua_l_tolstring(l, idx, &mut len);
        lua_l_addvalue(b);
      }
    }
  }
}
