use crate::{
  enums::lua_type::LuaType, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 校验/取长/压栈全经安全门面，体内已无真实裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错的受保护帧——
/// 栈 1 号位为 table（`check_type` 校验、非表即抛错发散），`obj_len`/`push_integer` 读取该栈槽并可
/// 触发 GC/分配。cpp/VM/src/ltablib.cpp:83 getn。
pub fn getn(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);

  l.push_integer(l.obj_len(1) as i32);

  1
}

lua_lib_fn!(pub fn getn @ref, getn_arm);
