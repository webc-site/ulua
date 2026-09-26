use crate::{
  functions::{
    class_classof::class_classof_arm, class_isinstance::class_isinstance_arm,
    lua_l_register::lua_l_register,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 `LuaState`；`luaL_register` 会建模块表并逐个注册 `class_lib` 各项（分配、可抛错），
/// 须在受保护帧内由 C 侧以合法 `LuaState*` 调入；`class_lib` 为静态空名终止表。cpp `lclasslib.cpp:63`。
pub(crate) unsafe fn luaopen_class(l: *mut LuaState) -> i32 {
  unsafe {
    let class_lib: [LuaLReg; 2] = [
      LuaLReg::new(b"isinstance", class_isinstance_arm),
      LuaLReg::new(b"classof", class_classof_arm),
    ];

    lua_l_register(l, c"class".as_ptr(), &class_lib);
    1
  }
}

lua_lib_fn!(pub(crate) fn luaopen_class, luaopen_class_arm);
