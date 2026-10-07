//! `non_strict_context` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::collections::BTreeMap;

use crate::{
  functions::{
    collect_operands::collect_operands, simplify_intersection_simplify::simplify_intersection,
    simplify_union::simplify_union,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, non_strict_context::NonStrictContext,
    type_arena::TypeArena,
  },
  type_aliases::{def_id_def::DefId, type_id::TypeId},
};

impl NonStrictContext {
  pub fn add_context(&mut self, def: &DefId, ty: TypeId) {
    for def in collect_operands(*def) {
      self.context.insert(def, ty);
    }
  }
}

fn non_strict_context_conjunction(
  builtins: Handle<BuiltinTypes>,
  arena: Handle<TypeArena>,
  left: &NonStrictContext,
  right: &NonStrictContext,
) -> NonStrictContext {
  let mut conj = NonStrictContext {
    context: BTreeMap::new(),
  };

  for (&def, &left_ty) in &left.context {
    if let Some(right_ty) = right.find_def(def) {
      let result = simplify_intersection(builtins, arena, left_ty, right_ty);
      conj.context.insert(def, result.result);
    }
  }

  conj
}

pub(crate) fn non_strict_context_disjunction(
  builtin_types: Handle<BuiltinTypes>,
  arena: Handle<TypeArena>,
  left: &NonStrictContext,
  right: &NonStrictContext,
) -> NonStrictContext {
  let mut disj = NonStrictContext {
    context: BTreeMap::new(),
  };

  for (&def, &left_ty) in &left.context {
    if let Some(right_ty) = right.find_def(def) {
      let result = simplify_union(builtin_types, arena, left_ty, right_ty).result;
      disj.context.insert(def, result);
    } else {
      disj.context.insert(def, left_ty);
    }
  }

  for (&def, &right_ty) in &right.context {
    if left.find_def(def).is_none() {
      disj.context.insert(def, right_ty);
    }
  }

  disj
}

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

impl NonStrictContext {
  pub fn new() -> Self {
    Self::default()
  }
}

impl NonStrictContext {
  pub fn remove(&mut self, def: &DefId) -> bool {
    let mut result = true;
    for def in collect_operands(*def) {
      let erased = self.context.remove(&def);
      result = result && erased.is_some();
    }
    result
  }
}
