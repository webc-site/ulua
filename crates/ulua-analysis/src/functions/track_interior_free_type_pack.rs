use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{scope::Scope, scope_registry::resolve_scope_mut},
  type_aliases::type_pack_id::TypePackId,
};
pub fn track_interior_free_type_pack(scope: *mut Scope, tp: TypePackId) {
  LUAU_ASSERT!(!tp.is_null());

  let mut current_scope = scope;
  while !current_scope.is_null() {
    unsafe {
      if let Some(ref mut interior_free_type_packs) = (*current_scope).interior_free_type_packs {
        interior_free_type_packs.push(tp);
        return;
      }

      // 句柄化上溯：parent 为 ScopeId，写回下一节点的可变视图经 resolve_scope_mut
      // 取得（scope_registry 写回纪律，同一时刻至多一个存活 &mut）。
      current_scope = match (*current_scope).parent {
        Some(parent_id) => {
          resolve_scope_mut(parent_id).map_or_else(null_mut, |parent| parent as *mut Scope)
        }
        None => null_mut(),
      };
    }
  }

  LUAU_ASSERT!(false);
}
