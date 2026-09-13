use alloc::string::{String, ToString};

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{metatable_type::MetatableType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

/// C++ `getName(TypeId)` returns a `std::optional<Name>` (an owned string), not a
/// borrow. The previous port returned `&'static str` via `Box::leak`, leaking a
/// string on every call (the checker resolves names while stringifying errors) —
/// caught by the fuzz suite's LeakSanitizer. Return an owned `String`; the sole
/// caller only compares it.
pub fn get_name(type_id: TypeId) -> Option<String> {
  let mut ty = follow_type_id(type_id);

  if let Some(mtv) = get_type_id::<MetatableType>(ty) {
    if let Some(name) = mtv.synthetic_name() {
      return Some(name.to_string());
    }
    ty = follow_type_id(mtv.table());
  }

  if let Some(ttv) = get_type_id::<TableType>(ty) {
    if let Some(name) = &ttv.name {
      return Some(name.clone());
    }
    if let Some(name) = &ttv.synthetic_name {
      return Some(name.clone());
    }
  }

  None
}
