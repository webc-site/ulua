use core::{ffi::c_char, mem::size_of};

use crate::{records::lua_state::LuaState, type_aliases::t_value::TValue};

/// 判断 `LuaState` 距栈顶余量是否已达 `n` 个 `TValue`。仅读 `stack_last`/`top` 两个
/// 指针字段的地址做有符号差值，不解引用，故以 `&LuaState` 接收者替代原 `*mut` 裸指针。
pub fn stacklimitreached(l: &LuaState, n: i32) -> bool {
  let stack_last = l.stack_last as *mut c_char;
  let top = l.top as *mut c_char;
  // C++ does SIGNED pointer subtraction (`ptrdiff_t`): once `top` passes
  // `stack_last`, the difference goes negative → the limit is reached. The
  // original `as usize` subtraction underflowed when `top > stack_last` —
  // a panic with overflow-checks (fuzz build), and in release it wraps to a
  // huge value, wrongly reporting "limit NOT reached". Compute it signed to
  // match C++. (Found by the `splice` fuzz target — a deeply recursive
  // program pushes `top` past `stack_last`.)
  let diff = (stack_last as isize) - (top as isize);
  let threshold = (n as isize) * (size_of::<TValue>() as isize);
  diff <= threshold
}
