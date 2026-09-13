use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::scope::Scope, type_aliases::type_pack_id::TypePackId};
pub fn track_interior_free_type_pack(scope: *mut Scope, tp: TypePackId) {
  LUAU_ASSERT!(!tp.is_null());

  let mut current_scope = scope;
  while !current_scope.is_null() {
    unsafe {
      if let Some(ref mut interior_free_type_packs) = (*current_scope).interior_free_type_packs {
        interior_free_type_packs.push(tp);
        return;
      }

      if let Some(ref parent_ptr) = (*current_scope).parent {
        current_scope = parent_ptr.as_ref() as *const Scope as *mut Scope;
      } else {
        current_scope = null_mut();
      }
    }
  }

  LUAU_ASSERT!(false);
}
