use crate::{
  enums::lua_type::LuaType, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 类型校验、`lua_next`/`lua_call` 全经安全门面转调，体内已无真实裸操作，故本体降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧——栈 index 1 为 table、index 2 为 function（`check_type` 校验，
/// 不符即抛错发散）；循环内 `next`/`call` 读写栈、可 GC、可抛错，`call(2, 1)` 需 `top` 后留结果余量。
/// cpp `ltablib.cpp:35`。
pub fn foreach(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);
  l.check_type(2, LuaType::Function);
  l.push_nil(); // first key
  while l.next(1) {
    l.push_value(2); // function
    l.push_value(-3); // key
    l.push_value(-3); // value
    l.call(2, 1);
    if !l.is_nil(-1) {
      return 1;
    }
    l.pop(2); // remove value and result
  }
  0
}

lua_lib_fn!(pub fn foreach @ref, foreach_arm);
