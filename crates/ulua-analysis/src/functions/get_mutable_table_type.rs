use crate::{
  functions::{follow_type, get_mutable_type, get_type},
  records::{metatable_type::MetatableType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn get_mutable_table_type(type_id: TypeId) -> Option<&'static mut TableType> {
  let ty = follow_type::follow(type_id);

  get_mutable_type::get_mutable::<TableType>(ty).or_else(|| {
    let mtv = get_type::get::<MetatableType>(ty)?;
    get_mutable_type::get_mutable::<TableType>(follow_type::follow(mtv.table()))
  })
}
