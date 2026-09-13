use crate::{
  functions::{
    flatten_intersection::flatten_intersection, follow_type::follow_type_id, get_type_alt_j::get,
  },
  records::{intersection_type::IntersectionType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn is_table_intersection(ty: TypeId) -> bool {
  if get::<IntersectionType>(follow_type_id(ty)).is_none() {
    return false;
  }

  let parts = flatten_intersection(ty);
  parts.iter().all(|&part| !get::<TableType>(part).is_none())
}
