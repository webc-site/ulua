use crate::{
  functions::lua_isyieldable::lua_isyieldable, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn coyieldable(l: *mut LuaState) -> i32 {
  unsafe {
    // r16-v3 #60 拆两语句：`lua_isyieldable` 前移 `&LuaState` 只读形后，原单语句的
    // `(*l).push_boolean(&…)` 门面独占接收者与只读实参借用冲突；原位现读（求值次序
    // 本就先读计数后落笔，句间无场写）拆分等价。
    let yieldable = lua_isyieldable(&*l) != 0;
    (*l).push_boolean(yieldable);
    1
  }
}

lua_lib_fn!(pub fn coyieldable, coyieldable_arm);
