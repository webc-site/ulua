use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v40 收形后
/// 取参/判空/压栈全经安全门面，体内无裸操作，故降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 1 号位经 `check_number` 要求存在且数值（非数字抛错发散），2 号位可选
/// （`is_none_or_nil` 判空，缺省走自然对数；否则经 `check_number` 取底数）；`push_number` 需 `l` 的
/// 栈顶后 ≥1 空槽；可触发 GC。cpp VM/src/lmathlib.cpp:146
pub(crate) fn math_log(l: &mut LuaState) -> i32 {
  let x = l.check_number(1);
  let res = if l.is_none_or_nil(2) {
    x.ln()
  } else {
    let base = l.check_number(2);
    if base == 2.0 {
      x.log2()
    } else if base == 10.0 {
      x.log10()
    } else {
      x.ln() / base.ln()
    }
  };

  l.push_number(res);
  1
}

lua_lib_fn!(pub(crate) fn math_log @ref, math_log_arm);
