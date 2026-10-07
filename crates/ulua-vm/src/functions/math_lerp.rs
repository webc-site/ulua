use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v40 收形后
/// 取参/压栈全经安全门面，体内无裸操作，故降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 1/2/3 号位按序经 `check_number` 强制为数字（非数字抛错发散），
/// 结果经 `push_number` 写回。cpp/VM/src/lmathlib.cpp:438 math_lerp。
pub fn math_lerp(l: &mut LuaState) -> i32 {
  let a = l.check_number(1);
  let b = l.check_number(2);
  let t = l.check_number(3);

  let r = if t == 1.0 { b } else { a + (b - a) * t };

  l.push_number(r);
  1
}

lua_lib_fn!(pub fn math_lerp @ref, math_lerp_arm);
