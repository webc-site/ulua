//! `fragment_autocomplete_type_cloner` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use crate::{
  records::{
    arena_handle::Handle,
    builtin_types::BuiltinTypes,
    fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner,
    scope::Scope,
    type_arena::TypeArena,
    type_cloner::{SeenTypePacks, SeenTypes, TypeCloner},
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

// Source: `Analysis/src/Clone.cpp:478-491`
// `FragmentAutocompleteTypeCloner::FragmentAutocompleteTypeCloner(...)`.

impl<'a> FragmentAutocompleteTypeCloner<'a> {
  /// cpp 子类构造只多带一枚 `replacementForNullScope`;基类的 `forceTy`/`forceTp`
  /// 在本重载的全部调用点恒为 `nullptr`(无强制复用节点),故不入形参、直接置
  /// `None`。`fresh_scope_for_free_types` 用 `&mut` 承接 cpp 的非空 `Scope*` 前置
  /// 条件(原 `LUAU_ASSERT!(!p.is_null())` 由类型排除),经 [`Handle`] 存为别名句柄。
  pub fn new(
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
    types: &'a mut SeenTypes,
    packs: &'a mut SeenTypePacks,
    fresh_scope_for_free_types: &mut Scope,
  ) -> Self {
    Self {
      // `TypeCloner(arena, builtinTypes, types, packs, forceTy, forceTp)`.
      // The override state (Clone.cpp:493-518, 541-544) is carried on the
      // base so the shared clone machinery applies it to the whole subgraph.
      base: TypeCloner {
        arena,
        builtin_types,
        queue: Vec::new(),
        types,
        packs,
        force_ty: None,
        force_tp: None,
        steps: 0,
        replacement_for_null_scope: Some(Handle::from_mut(fresh_scope_for_free_types)),
        skip_lazy_type_clone: true,
      },
    }
  }
}

impl FragmentAutocompleteTypeCloner<'_> {
  pub fn shallow_clone_type_id(&mut self, ty: TypeId) -> TypeId {
    self.base.shallow_clone_type_id(ty)
  }

  pub fn shallow_clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    self.base.shallow_clone_type_pack_id(tp)
  }
}
