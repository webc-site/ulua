//! Source: `Analysis/src/TypeInfer.cpp`
//!
//! C++ `struct Demoter : Substitution`（TypeInfer.cpp:773）——基类子对象由
//! `base: Substitution` 承接，虚函数覆写经 [`crate::records::tarjan::SubstitutionVtable`]
//! 挂接（见 `methods/demoter_demoter.rs`），与 `Replacer` 同款模式。

use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes, substitution::Substitution,
  type_arena::TypeArena,
};

#[derive(Debug, Clone)]
pub struct Demoter {
  pub(crate) base: Substitution,
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) builtins: Handle<BuiltinTypes>,
}
