use crate::{
  methods::{
    non_strict_context_conjunction::non_strict_context_conjunction,
    non_strict_context_disjunction::non_strict_context_disjunction,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, non_strict_context::NonStrictContext,
    type_arena::TypeArena,
  },
  type_aliases::{def_id_def::DefId, type_id::TypeId},
};

impl NonStrictContext {
  pub fn find_def_id(&self, def: &DefId) -> Option<TypeId> {
    self.find_def(*def)
  }

  pub fn disjunction(
    builtin_types: Handle<BuiltinTypes>,
    arena: Handle<TypeArena>,
    left: &NonStrictContext,
    right: &NonStrictContext,
  ) -> NonStrictContext {
    // 形参沿用 C++ `NotNull<BuiltinTypes/TypeArena>` 契约（调用点均出自
    // NonStrictTypeChecker 构造期接线的会话级实例句柄），null 由类型编码排除。
    non_strict_context_disjunction(builtin_types, arena, left, right)
  }

  pub fn conjunction(
    builtin_types: Handle<BuiltinTypes>,
    arena: Handle<TypeArena>,
    left: &NonStrictContext,
    right: &NonStrictContext,
  ) -> NonStrictContext {
    // 同 disjunction——NotNull 契约由句柄类型编码保证。
    non_strict_context_conjunction(builtin_types, arena, left, right)
  }

  pub fn find_def(&self, d: DefId) -> Option<TypeId> {
    self.context.get(&d).copied()
  }
}
