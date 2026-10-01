//! Source: `Analysis/src/Clone.cpp:25` (hand-ported; fields only)
//! Clone.h:15: using SeenTypes = std::unordered_map<TypeId, TypeId>;
//! Clone.h:16: using SeenTypePacks = std::unordered_map<TypePackId, TypePackId>;

use alloc::vec::Vec;

use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, scope::Scope, type_arena::TypeArena,
  },
  type_aliases::{
    collections::HashMap, type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId,
  },
};

pub type SeenTypes = HashMap<TypeId, TypeId>;
pub type SeenTypePacks = HashMap<TypePackId, TypePackId>;

/// 克隆会话状态。
///
/// `types`/`packs` 直译 cpp 的引用成员 `SeenTypes& types`:会话期间借用、从不
/// 拥有,故用带生命周期的独占引用而非裸指针(review.md §2)。借用同时消灭原先
/// 逐处的 `(*self.types).insert(..)` 解引用及其 SAFETY 契约——非空与存活期改由
/// 类型系统承载。
#[derive(Debug)]
pub struct TypeCloner<'a> {
  pub arena: Handle<TypeArena>,
  pub builtin_types: Handle<BuiltinTypes>,
  pub queue: Vec<TypeOrPack>,
  pub types: &'a mut SeenTypes,
  pub packs: &'a mut SeenTypePacks,
  /// cpp `forceTy`/`forceTp`(默认 `nullptr` = 无强制复用节点),可空用 `Option`。
  pub force_ty: Option<TypeId>,
  pub force_tp: Option<TypePackId>,
  pub steps: i32,
  /// Subclass state carried by `FragmentAutocompleteTypeCloner` (Clone.cpp:473).
  /// In C++ the fragment cloner is a `TypeCloner` subclass whose `shallowClone`
  /// override (Clone.cpp:493-518) substitutes this scope for a null free/table
  /// scope, and whose `cloneChildren(LazyType*)` override (Clone.cpp:541-544)
  /// is a no-op. Because the recursive `cloneChildren` machinery dispatches
  /// through the virtual `shallowClone`, that substitution must apply to every
  /// node cloned in the subgraph, not just the root. The Rust port has no
  /// vtable, so the divergence is encoded as cloner state read by the (shared)
  /// `shallowClone`/`cloneChildren` methods. Both fields are inert for the
  /// non-fragment callers (no scope == prior behaviour, skip flag == false).
  ///
  /// cpp `Scope* replacementForNullScope` 的可空别名用 `Option<Handle<_>>` 表达,
  /// 仅在写入 `FreeType::scope` 这类契约字段时折算为裸指针。
  pub replacement_for_null_scope: Option<Handle<Scope>>,
  pub skip_lazy_type_clone: bool,
}
