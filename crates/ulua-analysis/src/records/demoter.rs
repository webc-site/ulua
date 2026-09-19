//! Source: `Analysis/src/TypeInfer.cpp`

use crate::records::{builtin_types::BuiltinTypes, type_arena::TypeArena};
#[derive(Debug, Clone)]
pub struct Demoter {
  pub(crate) arena: *mut TypeArena,
  pub(crate) builtins: *mut BuiltinTypes,
}
