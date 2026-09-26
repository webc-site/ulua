//! Source: `VM/src/lcorolib.cpp:14`
//!
//! `coroutine.status` — push the textual status of the thread argument. The
//! index order matches `lua_costatus`: 0 running, 1 suspended, 2 normal, 3/4
//! dead (the C++ `statnames` table repeats "dead" for COERR/COFIN).

use crate::{
  enums::lua_co_status::LuaCoStatus,
  functions::{
    lua_costatus::lua_costatus, lua_pushstring::lua_pushstring, lua_tothread::lua_tothread,
  },
  macros::{lua_l_argexpected::luaL_argexpected, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub(crate) unsafe fn costatus(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `L` 存活且实参 1 为 thread 槽位，读取其 status 字段不越出协程对象界
  unsafe {
    let co = lua_tothread(l, 1);
    luaL_argexpected!(l, co.is_some(), 1, "thread");
    let co = co.expect("luaL_argexpected 已证 co 非空");

    let name = LuaCoStatus::from_c_int(lua_costatus(l, co))
      .map_or(b"dead\0".as_slice(), LuaCoStatus::as_bytes);
    lua_pushstring(l, name.as_ptr().cast());
    1
  }
}

lua_lib_fn!(pub(crate) fn costatus, costatus_arm);
