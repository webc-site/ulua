use crate::{
  functions::{
    lua_a_pushclass::lua_a_pushclass, lua_a_toobject::lua_a_toobject,
    lua_l_checkany::lua_l_checkany, lua_pushnil::lua_pushnil,
  },
  macros::{lua_isobject::lua_isobject, lua_lib_fn::lua_lib_fn, objectvalue::objectvalue},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈 index 1 存在任意值（`luaL_checkany`）；当其为对象实例时
/// `lua_a_toobject` 返回指向该 TValue 的非空指针，其 `value.gc` 须为存活 `LuauObject`（读 `.lclass`），
/// `luaA_pushclass` 可分配，须在受保护帧内调用。cpp `lclasslib.cpp:40`。
pub unsafe fn class_classof(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);

    if !lua_isobject!(l, 1) {
      lua_pushnil(l);
      return 1;
    }

    let inst: *const TValue = lua_a_toobject(l, 1);
    let ci = objectvalue!(inst);
    lua_a_pushclass(l, (*ci).lclass);
    1
  }
}

lua_lib_fn!(pub fn class_classof, class_classof_arm);
