use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_a_toobject::lua_a_toobject, lua_l_checkany::lua_l_checkany,
    lua_l_checktype::lua_l_checktype, lua_pushboolean::lua_pushboolean,
  },
  macros::{classvalue::classvalue, objectvalue::objectvalue},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkany(l,1)`、`luaL_checktype(l,2,CLASS)` 要求索引 2 为 class
/// 否则抛错回退；`lua_a_toobject(l,1/2)` 返回栈内 TValue 裸指针（索引须存在）；`obj` 侧 `classvalue!` 与 `inst` 侧
/// 沿 `(*LuauObject).lclass.super_` 上溯的链均须为存活 class 对象；`lua_pushboolean` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lclasslib.cpp:10
pub unsafe extern "C-unwind" fn class_isinstance(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);
    lua_l_checktype(l, 2, LuaType::Class as i32);

    let inst: *const TValue = lua_a_toobject(l, 1);
    let obj: *const TValue = lua_a_toobject(l, 2);

    // classvalue!/objectvalue! 已返回类型化裸指针（对应 cpp 的 LuauClass*/LuauObject*）
    let lclass = classvalue!(obj);

    if !(*inst).is_object() {
      lua_pushboolean(l, 0);
      return 1;
    }

    let obj_ptr = objectvalue!(inst);
    let mut obj_class = (*obj_ptr).lclass;

    while !obj_class.is_null() {
      if obj_class == lclass {
        lua_pushboolean(l, 1);
        return 1;
      }
      obj_class = (*obj_class).super_;
    }

    lua_pushboolean(l, 0);
    1
  }
}
