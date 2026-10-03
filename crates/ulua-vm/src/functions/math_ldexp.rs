use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v40 收形后
/// 取参/压栈全经安全门面，体内无裸操作，故降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 1 号位经 `check_number` 取数字、2 号位经 `check_integer`
/// 取整数指数（非对应类型抛错发散），结果经 `push_number` 写回。cpp/VM/src/lmathlib.cpp:198 math_ldexp。
pub fn math_ldexp(l: &mut LuaState) -> i32 {
  let x = l.check_number(1);
  let exp = l.check_integer(2);
  // ldexp(x, exp) is x * 2^exp
  l.push_number(x * (2.0f64).powi(exp));
  1
}

lua_lib_fn!(pub fn math_ldexp @ref, math_ldexp_arm);
