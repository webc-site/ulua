use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{arena_handle::alias_opt_mut, scope::Scope, scope_registry::resolve_scope_mut},
  type_aliases::type_id::TypeId,
};
pub fn track_interior_free_type(scope: *mut Scope, ty: TypeId) {
  // §2：null 哨兵改为 `Option<&mut Scope>` 沿 parent 句柄链上溯；解引用收口于
  // `alias_opt_mut` 单点，parent 为 `ScopeId`，下一节点经 resolve_scope_mut 取回
  // （scope_registry 写回纪律，同一时刻至多一个存活 `&mut`）。
  let mut current = alias_opt_mut(scope);
  while let Some(sc) = current.take() {
    if let Some(interior_free_types) = &mut sc.interior_free_types {
      interior_free_types.push(ty);
      return;
    }

    let parent_id = sc.parent;
    current = parent_id.and_then(resolve_scope_mut);
  }

  LUAU_ASSERT!(false);
}
