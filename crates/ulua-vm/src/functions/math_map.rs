use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v40 收形后
/// 取参/压栈全经安全门面，体内无裸操作，故降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 1..=5 号位按序逐个经 `check_number` 强制为数字
/// （非数字抛错发散），结果经 `push_number` 写回并可分配/GC。cpp/VM/src/lmathlib.cpp:425 math_map。
pub fn math_map(l: &mut LuaState) -> i32 {
  let x = l.check_number(1);
  let inmin = l.check_number(2);
  let inmax = l.check_number(3);
  let outmin = l.check_number(4);
  let outmax = l.check_number(5);

  let result = outmin + (x - inmin) * (outmax - outmin) / (inmax - inmin);
  l.push_number(result);
  1
}

lua_lib_fn!(pub fn math_map @ref, math_map_arm);
