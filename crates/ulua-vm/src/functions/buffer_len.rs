use crate::{
  functions::buffer_window::buffer_data_ref, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全）：以 buffer 库 C 函数约定被调——`l` 存活与独占由
/// `&mut LuaState` 承载，实参 1 为已检查 buffer userdata，结果数值压栈需栈顶余量。
pub(crate) fn buffer_len(l: &mut LuaState) -> i32 {
  let buf = buffer_data_ref(l, 1);
  l.push_number(buf.len() as f64);

  1
}

lua_lib_fn!(pub(crate) fn buffer_len @ref, buffer_len_arm);
