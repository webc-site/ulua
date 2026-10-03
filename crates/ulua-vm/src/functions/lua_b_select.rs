use crate::{
  enums::lua_type::LuaType, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载，r16-v29 收形后
/// 体内全经安全门面方法，无裸操作，降为安全 `fn`）：
/// `l` 须处于受保护帧且 `lua_gettop` 取实参数 n（须 ≥1）；若索引 1 为字符串则
/// 读其首字节判 '#'（该串槽存活）；否则 `check_integer(l,1)` 要求可转整数，
/// `arg_check` 校验归一化下标 1≤i 越界即抛错回退；走 '#' 分支时 `push_integer` 需 `top` 后 ≥1 空槽。
/// cpp VM/src/lbaselib.cpp:265
pub(crate) fn lua_b_select(l: &mut LuaState) -> i32 {
  let n = l.get_top();
  let first_type = l.type_of(1);
  if first_type == LuaType::String
    && let Some(bytes) = l.to_bytes(1)
    && bytes.first() == Some(&b'#')
  {
    l.push_integer(n - 1);
    return 1;
  }

  let i = l.check_integer(1);
  let i = if i < 0 {
    n + i
  } else if i > n {
    n
  } else {
    i
  };

  l.arg_check(1 <= i, 1, "index out of range");
  n - i
}

lua_lib_fn!(pub(crate) fn lua_b_select @ref, lua_b_select_arm);
