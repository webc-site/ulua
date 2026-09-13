use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{metatable_type::MetatableType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn get_table_type(type_id: TypeId) -> Option<&'static TableType> {
  let mut ty = follow_type_id(type_id);

  if let Some(ttv) = get_type_id::<TableType>(ty) {
    return Some(ttv);
  }

  if let Some(mtv) = get_type_id::<MetatableType>(ty) {
    ty = follow_type_id(mtv.table());

    if let Some(ttv) = get_type_id::<TableType>(ty) {
      return Some(ttv);
    }
  }

  None
}
