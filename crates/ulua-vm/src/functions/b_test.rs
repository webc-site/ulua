use crate::{
  functions::andaux::andaux, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// binary32 `btest` 核心：[`andaux`] 折叠非零判定、布尔压栈。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调（binary32 C 函数面），
/// 实参越界或非数值由 check*/argerror 报错，结果压栈需栈顶留有 LUA_MINSTACK 余量；
/// `l` 存活与独占由 `&mut LuaState` 承载。
pub(crate) fn b_test(l: &mut LuaState) -> i32 {
  let r = andaux(l);
  l.push_boolean(r != 0);
  1
}

lua_lib_fn!(pub(crate) fn b_test @ref, b_test_arm);
