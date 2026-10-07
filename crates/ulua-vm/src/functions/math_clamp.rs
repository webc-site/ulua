use core::cmp::Ordering::{Equal, Less};

use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v40 收形后
/// 取参/校验/压栈全经安全门面，体内无裸操作，故降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 1/2/3 号位按序经 `check_number` 强制为数字（非数字抛错发散），
/// 随后 `arg_check` 校验 `min <= max`（不满足则以 3 号位报 "max must be greater than or equal to
/// min" 发散），结果经 `push_number` 写回。cpp/VM/src/lmathlib.cpp:397 math_clamp。
pub fn math_clamp(l: &mut LuaState) -> i32 {
  let v = l.check_number(1);
  let min = l.check_number(2);
  let max = l.check_number(3);

  l.arg_check(
    matches!(min.partial_cmp(&max), Some(Less | Equal)),
    3,
    "max must be greater than or equal to min",
  );

  let r = if v < min { min } else { v };
  let r = if r > max { max } else { r };

  l.push_number(r);
  1
}

lua_lib_fn!(pub fn math_clamp @ref, math_clamp_arm);
