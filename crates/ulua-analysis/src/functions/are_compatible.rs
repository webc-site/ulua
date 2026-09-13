use ulua_common::macros::luau_assert::LUAU_ASSERT;

// Two tables may be compatible even if their shapes aren't exactly the
// same if the extra property is optional, free (and therefore
// potentially optional), or if the right table has an indexer.  Or if
// the right table is free (and therefore potentially has an indexer or
// a compatible property)
use crate::enums::table_state::TableState;
use crate::{
  functions::{follow_type::follow_type_id, get_2::get2, is_optional_or_free::is_optional_or_free},
  records::{property_type::Property, table_type::TableType},
  type_aliases::type_id::TypeId,
};
unsafe fn missing_prop_is_compatible(left_prop: &Property, right_table: &TableType) -> bool {
  if right_table.state == TableState::Free || right_table.indexer.is_some() {
    return true;
  }

  if (left_prop.is_read_only() || left_prop.is_shared())
    && is_optional_or_free(left_prop.read_ty.unwrap())
  {
    return true;
  }

  // FIXME: Could this create an issue for write only / divergent properties?
  false
}

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
pub unsafe fn are_compatible(left: TypeId, right: TypeId) -> bool {
  unsafe {
    let p = get2::<TableType, TableType, TypeId>(follow_type_id(left), follow_type_id(right));
    if p.first.is_null() {
      return true;
    }

    let left_table = p.first;
    LUAU_ASSERT!(!left_table.is_null());
    let right_table = p.second;
    LUAU_ASSERT!(!right_table.is_null());

    let left_table = &*left_table;
    let right_table = &*right_table;

    for (_name, left_prop) in left_table.props.iter() {
      let it = right_table.props.get(_name);
      if it.is_none() && !missing_prop_is_compatible(left_prop, right_table) {
        return false;
      }
    }

    for (_name, right_prop) in right_table.props.iter() {
      let it = left_table.props.get(_name);
      if it.is_none() && !missing_prop_is_compatible(right_prop, left_table) {
        return false;
      }
    }

    true
  }
}
