use core::{ffi::c_char, ptr::null};

use memchr::memchr;

use crate::{
  functions::{
    cstr_bytes, lua_createtable::lua_createtable, lua_pushlstring::lua_pushlstring_bytes,
    lua_rawget::lua_rawget, lua_settable::lua_settable,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须指向存活的 `LuaState`；`fname` 为非空字节切片。
pub unsafe fn lua_l_findtable_bytes(
  l: *mut LuaState,
  idx: i32,
  mut fname: &[u8],
  szhint: i32,
) -> *const u8 {
  unsafe {
    (*l).push_value(idx);
    loop {
      let dot = memchr(b'.', fname);
      let len = dot.unwrap_or(fname.len());
      let seg = &fname[..len];

      lua_pushlstring_bytes(l, seg);
      lua_rawget(l, -2);

      if (*l).is_nil(-1) {
        (*l).pop(1); // remove this nil
        let next_szhint = if dot.is_some() { 1 } else { szhint };
        lua_createtable(l, 0, next_szhint);
        lua_pushlstring_bytes(l, seg);
        (*l).push_value(-2);
        lua_settable(l, -4);
      } else if !(*l).is_table(-1) {
        (*l).pop(2); // remove table and value
        return fname.as_ptr();
      }

      (*l).remove(-2); // remove previous table

      match dot {
        Some(i) => fname = &fname[i + 1..],
        // 段尾无 '.'：与 cpp `*e != '.'` break 一致
        None => return null(),
      }
    }
  }
}

/// `const char* lua_l_findtable(LuaState* l, int idx, const char* fname, int szhint)`
///
/// C++ source: `VM/src/laux.cpp:330`
/// # Safety
/// `l` 必须指向存活 `LuaState` 且 `idx` 为合法栈索引（每步 getfield/setfield 在当前栈顶进行，遇非表中间段
/// 会把该段起点裸指针返回——`fname` 须在返回前保持可读）；`fname` 为 NUL 结尾 C 串路径，`szhint` 仅影响新建
/// 表预分配。对应 cpp laux.cpp:330。
pub unsafe fn lua_l_findtable(
  l: *mut LuaState,
  idx: i32,
  fname: *const c_char,
  szhint: i32,
) -> *const c_char {
  unsafe {
    let bytes = cstr_bytes(fname);
    let ret = lua_l_findtable_bytes(l, idx, bytes, szhint);
    ret.cast::<c_char>()
  }
}
