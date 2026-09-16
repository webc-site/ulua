use crate::{
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
  },
  records::{metatable_type::MetatableType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn get_mutable_table_type(type_id: TypeId) -> Option<&'static mut TableType> {
  let ty = follow_type_id(type_id);

  get_mutable_type_id::<TableType>(ty).or_else(|| {
    let mtv = get_type_id::<MetatableType>(ty)?;
    get_mutable_type_id::<TableType>(follow_type_id(mtv.table()))
  })
}
