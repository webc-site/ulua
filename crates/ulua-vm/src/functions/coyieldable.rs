use crate::{
  functions::lua_isyieldable::lua_isyieldable, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 判读/压栈全经安全门面，体内已无裸操作，故本体降为安全 `fn`）：`l` 须为正被本线程驱动的存活
/// `LuaState`（`lua_isyieldable` 只读其 `n_ccalls`/`base_ccalls` 两计数），`push_boolean` 占用
/// top 之上 1 个空槽。cpp/VM/src/lcorolib.cpp:361 coyieldable。
pub fn coyieldable(l: &mut LuaState) -> i32 {
  l.push_boolean(lua_isyieldable(l) != 0);

  1
}

lua_lib_fn!(pub fn coyieldable @ref, coyieldable_arm);
