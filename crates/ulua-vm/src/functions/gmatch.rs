use crate::{
  functions::gmatch_aux::gmatch_aux_arm, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 校验/截栈/压整数全经安全门面，仅登记闭包一次为不安全调用而落窄块，故本体降为安全 `fn`）：`l`
/// 须处于可抛错的受保护帧——索引 1/2 须为字符串（`check_bytes` 校验、否则抛错发散）；`set_top(2)`
/// 截栈后经 `push_c_closure` 连压源串/模式串/游标整数 3 个 upvalue 建 `gmatch_aux` 迭代闭包。
pub fn gmatch(l: &mut LuaState) -> i32 {
  l.check_bytes(1);
  l.check_bytes(2);
  l.set_top(2);
  l.push_integer(0);
  // §10：`None` 即原 `null()` 空 debugname 契约位；被登记 `_arm` 遵循 Lua C 函数约定，3 个 upvalue 已由上方
  // set_top/push_integer 与栈上 1/2 号位备妥。
  l.push_c_closure(Some(gmatch_aux_arm), None, 3);
  1
}

lua_lib_fn!(pub fn gmatch @ref, gmatch_arm);
