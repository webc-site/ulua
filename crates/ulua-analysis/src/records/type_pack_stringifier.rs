//! Node: `cxx:Record:Luau.Analysis:Analysis/src/ToString.cpp:1167:type_pack_stringifier`
//! Source: `Analysis/src/ToString.cpp:1167-1184` (hand-ported)
//!
//! C++ `struct TypePackStringifier` — `elemNames` is a by-value const member
//! in C++ (copied from the ctor ref), hence the owned Vec.

use alloc::vec::Vec;

use crate::records::{function_argument::FunctionArgument, stringifier_state::StringifierState};

#[derive(Debug)]
pub struct TypePackStringifier {
  pub(crate) state: *mut StringifierState,
  pub(crate) elem_names: Vec<Option<FunctionArgument>>,
  pub(crate) elem_index: u32,
}
