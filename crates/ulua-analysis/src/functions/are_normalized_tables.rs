use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    metatable_type::MetatableType, primitive_type::PrimitiveType, table_type::TableType,
    type_ids::TypeIds,
  },
};

pub fn are_normalized_tables(tys: &TypeIds) -> bool {
  for ty in tys.order.iter() {
    if !get_type_id::<TableType>(*ty).is_none() || !get_type_id::<MetatableType>(*ty).is_none() {
      continue;
    }

    if let Some(pt) = get_type_id::<PrimitiveType>(*ty).as_ref()
      && pt.r#type == PrimitiveType::TABLE
    {
      continue;
    }

    return false;
  }

  true
}
