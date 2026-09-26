//! Source: `Analysis/src/ToString.cpp:1167-1184` (hand-ported)
//!
//! C++ `struct TypePackStringifier` — `elemNames` is a by-value const member
//! in C++ (copied from the ctor ref), hence the owned Vec.

use alloc::vec::Vec;

use crate::records::{
  arena_handle::alias, function_argument::FunctionArgument, stringifier_state::StringifierState,
};

#[derive(Debug)]
pub struct TypePackStringifier {
  pub(crate) state: *mut StringifierState,
  pub(crate) elem_names: Vec<Option<FunctionArgument>>,
  pub(crate) elem_index: u32,
}

impl TypePackStringifier {
  /// §2 裸指针收口单点：契约同 [`TypeStringifier::st`]——`state` 指向入口
  /// `to_string*` 构造期接线的存活 `StringifierState`，解引用只发生在 [`alias`]。
  pub(crate) fn st(&mut self) -> &'static mut StringifierState {
    alias(self.state)
  }
}
