use crate::{
  enums::lua_type::LuaType, functions::lua_getreadonly::lua_getreadonly,
  macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 判型/读只读标志/压布尔全经安全门面，体内已无真实裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错
/// 的受保护帧——栈 1 号位为 table（`check_type` 校验、非表即抛错发散），`push_boolean` 需栈顶留结果余量。
///
/// 只读标志拆两语句现读（r16-v3 #60 同判例）：`lua_getreadonly` 收形为 `&LuaState` 只读形后与
/// `push_boolean` 独占接收者借用冲突；原位现读、句间无场写，拆句逐位等价。cpp/VM/src/ltablib.cpp:651。
pub fn tisfrozen(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);

  let frozen = lua_getreadonly(l, 1) != 0;

  l.push_boolean(frozen);

  1
}

lua_lib_fn!(pub fn tisfrozen @ref, tisfrozen_arm);
