use crate::{
  functions::{
    class_classof::class_classof_arm, class_isinstance::class_isinstance_arm,
    lua_l_register::lua_l_register_bytes,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// `class_lib[]`（cpp `lclasslib.cpp:63` 的 Lua 名 → `class_*` `_arm` 边界臂表）：
/// 全静态条目编译期落 `static`（rodata），开库时零栈构造、零写入。
static CLASS_LIB: [LuaLReg; 2] = [
  LuaLReg::new(b"isinstance", class_isinstance_arm),
  LuaLReg::new(b"classof", class_classof_arm),
];

/// # Safety
/// `l` 须为存活 `LuaState`；`lua_l_register_bytes` 会建模块表并逐个注册 `CLASS_LIB` 各项（分配、可抛错），
/// 须在受保护帧内由 C 侧以合法 `LuaState*` 调入；`CLASS_LIB` 为编译期静态表。cpp `lclasslib.cpp:63`。
pub(crate) unsafe fn luaopen_class(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register_bytes(l, Some(b"class"), &CLASS_LIB);
    1
  }
}

lua_lib_fn!(pub(crate) fn luaopen_class, luaopen_class_arm);
