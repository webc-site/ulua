use crate::{
  enums::lua_type::LuaType, functions::lua_getreadonly::lua_getreadonly,
  macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tisfrozen(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);

    // r16-v3 #60 拆两语句：`lua_getreadonly` 前移 `&LuaState` 只读形后与
    // `push_boolean` 独占接收者借用冲突；原位现读、句间无场写，拆句逐位等价。
    let frozen = lua_getreadonly(&*l, 1) != 0;

    (*l).push_boolean(frozen);

    1
  }
}

lua_lib_fn!(pub fn tisfrozen, tisfrozen_arm);
