use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{extern_type::ExternType, metatable_type::MetatableType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn type_could_have_metatable(ty: TypeId) -> bool {
  let followed = follow_type_id(ty);

  !get_type_id::<TableType>(followed).is_none()
    || !get_type_id::<ExternType>(followed).is_none()
    || !get_type_id::<MetatableType>(followed).is_none()
}
