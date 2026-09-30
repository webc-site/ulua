use crate::{
  functions::{follow_type, get_type},
  records::{extern_type::ExternType, metatable_type::MetatableType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn type_could_have_metatable(ty: TypeId) -> bool {
  let followed = follow_type::follow(ty);

  get_type::get::<TableType>(followed).is_some()
    || get_type::get::<ExternType>(followed).is_some()
    || get_type::get::<MetatableType>(followed).is_some()
}
