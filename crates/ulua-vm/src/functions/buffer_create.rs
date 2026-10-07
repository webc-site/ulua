use crate::{
  functions::lua_newbuffer::lua_newbuffer_push_ref, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r12-w6
/// 收形后取参/校验/分配压栈全经 `check_integer`/`arg_check`/`lua_newbuffer_push_ref` 安全门面，
/// 体内已无裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错、可分配/GC 的受保护帧——实参 1 经
/// `check_integer` 为非负数否则抛错发散，新 buffer 按该大小分配并压栈。cpp lbuflib.cpp `create`。
pub(crate) fn buffer_create(l: &mut LuaState) -> i32 {
  let size = l.check_integer(1);

  l.arg_check(size >= 0, 1, "size");

  lua_newbuffer_push_ref(l, size as usize);
  1
}

lua_lib_fn!(pub(crate) fn buffer_create @ref, buffer_create_arm);
