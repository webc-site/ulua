use crate::{
  functions::{flatten_intersection::flatten_intersection, follow_type, get_type::get},
  records::{intersection_type::IntersectionType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn is_table_intersection(ty: TypeId) -> bool {
  if get::<IntersectionType>(follow_type::follow(ty)).is_none() {
    return false;
  }

  let parts = flatten_intersection(ty);
  parts.iter().all(|&part| get::<TableType>(part).is_some())
}
