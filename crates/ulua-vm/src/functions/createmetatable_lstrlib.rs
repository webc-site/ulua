use crate::{
  functions::{
    lua_createtable::lua_createtable, lua_pushvalue::lua_pushvalue, lua_setfield::lua_setfield,
    lua_setmetatable::lua_setmetatable,
  },
  macros::{lua_pop::lua_pop, lua_pushliteral::lua_pushliteral, tm_index::TM_INDEX},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 且栈上已有字符串库表（相对索引 -2 指向它）；顶之上留足空槽供
/// `lua_createtable`/`lua_pushliteral`/`lua_pushvalue` 使用；全程可分配/GC，须在受保护帧调用。
/// cpp/VM/src/lstrlib.cpp:1667 createmetatable。
pub(crate) unsafe fn createmetatable_mut(l: *mut LuaState) {
  unsafe {
    lua_createtable(l, 0, 1); // create metatable for strings

    lua_pushliteral(l, b""); // dummy string

    lua_pushvalue(l, -2);

    lua_setmetatable(l, -2); // set string metatable

    lua_pop(l, 1); // pop dummy string

    lua_pushvalue(l, -2); // string library...

    lua_setfield(l, -2, TM_INDEX.as_ptr().cast()); // ...is the __index metamethod

    lua_pop(l, 1); // pop metatable
  }
}
