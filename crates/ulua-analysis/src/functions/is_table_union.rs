use crate::{
  functions::{follow_type, get_type::get},
  records::{table_type::TableType, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

pub fn is_table_union(ty: TypeId) -> bool {
  let followed = follow_type::follow(ty);
  let Some(ut) = get::<UnionType>(followed) else {
    return false;
  };

  ut.options
    .iter()
    .all(|&option| get::<TableType>(follow_type::follow(option)).is_some())
}
