use crate::{
  functions::lua_l_optnumber::lua_l_optnumber, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 取数/压栈全经安全门面，体内已无裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错的受保护帧——
/// 栈 1 号位为数字（`check_number` 非数字即抛错发散），2 号位为可选数字（`lua_l_optnumber` 缺席
/// 取 0.0），`push_number` 占用 top 之上 1 个空槽。cpp/VM/src/loslib.cpp:207 osdifftime。
pub fn os_difftime(l: &mut LuaState) -> i32 {
  let t1 = l.check_number(1);
  let t2 = lua_l_optnumber(l, 2, 0.0);

  // difftime in C returns the difference in seconds (t1 - t2) as a double.
  // Since we are targeting wasm32-unknown-unknown and portable environments,
  // and the input numbers are already doubles from the Lua stack, we can
  // perform the subtraction directly.
  // DELIBERATE DEVIATION：省去 cpp loslib.cpp:209 的 (time_t) 往返——输入
  // 已是 double，|t| < 2^53 逐位一致；超域为 C UB 角落，直取 double 相减。
  l.push_number(t1 - t2);
  1
}

lua_lib_fn!(pub fn os_difftime @ref, os_difftime_arm);
