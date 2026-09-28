use crate::{functions::lua_createtable::lua_createtable, records::lua_state::LuaState};

/// # Safety
/// `l` 须为存活 LuaState 且栈上已有字符串库表（相对索引 -2 指向它）；顶之上留足空槽供
/// `lua_createtable`/`lua_pushliteral`/`lua_pushvalue` 使用；全程可分配/GC，须在受保护帧调用。
/// cpp/VM/src/lstrlib.cpp:1667 createmetatable。
pub(crate) unsafe fn createmetatable_mut(l: *mut LuaState) {
  unsafe {
    lua_createtable(l, 0, 1); // create metatable for strings

    (*l).push_bytes(b""); // dummy string

    (*l).push_value(-2);

    (*l).set_metatable(-2); // set string metatable

    (*l).pop(1); // pop dummy string

    (*l).push_value(-2); // string library...

    (*l).set_field_str(-2, "__index"); // ...is the __index metamethod

    (*l).pop(1); // pop metatable
  }
}
