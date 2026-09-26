use core::{ffi::c_char, ptr::null};

use memchr::memchr;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    cstr_bytes, lua_createtable::lua_createtable, lua_pushlstring::lua_pushlstring,
    lua_pushvalue::lua_pushvalue, lua_rawget::lua_rawget, lua_remove::lua_remove,
    lua_settable::lua_settable, lua_type::lua_type,
  },
  macros::lua_pop::lua_pop,
  records::lua_state::LuaState,
};

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
  mut fname: *const c_char,
  szhint: i32,
) -> *const c_char {
  // Safety: 契约保证 `l` 存活且路径各步 getfield/setfield 均在当前栈顶进行，`n` 限定的步数与表结构一致
  unsafe {
    lua_pushvalue(l, idx);
    loop {
      // cpp 的 strchr/strlen 换成对 cstr_bytes 字节切片的单次扫描
      let bytes = cstr_bytes(fname);
      let dot = memchr(b'.', bytes);
      let len = dot.unwrap_or(bytes.len());

      lua_pushlstring(l, fname, len);
      lua_rawget(l, -2);

      if lua_type(l, -1) == (LuaType::Nil as i32) {
        lua_pop(l, 1); // remove this nil
        let next_szhint = if dot.is_some() { 1 } else { szhint };
        lua_createtable(l, 0, next_szhint);
        lua_pushlstring(l, fname, len);
        lua_pushvalue(l, -2);
        lua_settable(l, -4);
      } else if lua_type(l, -1) != (LuaType::Table as i32) {
        lua_pop(l, 2); // remove table and value
        return fname;
      }

      lua_remove(l, -2); // remove previous table

      match dot {
        Some(i) => fname = fname.add(i + 1),
        // 段尾无 '.'：与 cpp `*e != '.'` break 一致
        None => return null(),
      }
    }
  }
}
