use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get},
  records::{table_type::TableType, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

pub fn is_table_union(ty: TypeId) -> bool {
  let followed = follow_type_id(ty);
  let Some(ut) = get::<UnionType>(followed) else {
    return false;
  };

  ut.options
    .iter()
    .all(|&option| get::<TableType>(follow_type_id(option)).is_some())
}
