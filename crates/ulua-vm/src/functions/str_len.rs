use crate::{
  functions::lua_l_checklstring::lua_l_checklstring_ref, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v38 收形后
/// 体内全经安全门面/已收形被调，无裸操作，降为安全 `fn`）：
/// `l` 须处于可抛错受保护帧；栈槽 #1 为串实参（非串经 `lua_l_checklstring_ref` 抛
/// "string expected" 发散），`push_integer` 需 `top` 后 ≥1 空槽。
pub fn str_len(l: &mut LuaState) -> i32 {
  // 借用切片形态：出参 len 由切片长度直接承接
  let len = lua_l_checklstring_ref(l, 1).len();
  l.push_integer(len as i32);
  1
}

lua_lib_fn!(pub fn str_len @ref, str_len_arm);
