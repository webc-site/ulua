use std::ptr::eq;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{functions::get_type_alt_j::get_type_id, records::extern_type::ExternType};
pub fn is_subclass_extern_type_extern_type(cls: &ExternType, parent: &ExternType) -> bool {
  let mut current = cls;

  loop {
    if eq(current, parent) {
      return true;
    }

    let Some(parent_id) = current.parent else {
      return false;
    };

    // C++: parent = get<ExternType>(current->parent); LUAU_ASSERT(parent);
    let next = get_type_id::<ExternType>(parent_id);
    LUAU_ASSERT!(next.is_some());
    let Some(next) = next else { return false };
    current = next;
  }
}
