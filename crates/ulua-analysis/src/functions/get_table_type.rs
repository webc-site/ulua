use crate::{
  functions::{follow_type, get_type},
  records::{metatable_type::MetatableType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn get_table_type(type_id: TypeId) -> Option<&'static TableType> {
  let mut ty = follow_type::follow(type_id);

  if let Some(ttv) = get_type::get::<TableType>(ty) {
    return Some(ttv);
  }

  if let Some(mtv) = get_type::get::<MetatableType>(ty) {
    ty = follow_type::follow(mtv.table());

    if let Some(ttv) = get_type::get::<TableType>(ty) {
      return Some(ttv);
    }
  }

  None
}
