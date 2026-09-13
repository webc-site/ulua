use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType, free_type::FreeType, generic_type::GenericType,
    metatable_type::MetatableType, table_type::TableType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

pub fn can_suggest_inferred_type(ty: TypeId) -> bool {
  let ty = follow_type_id(ty);

  // No point in suggesting 'any', invalid to suggest others
  if get_type_id::<AnyType>(ty).is_some()
    || get_type_id::<ErrorType>(ty).is_some()
    || get_type_id::<GenericType>(ty).is_some()
    || get_type_id::<FreeType>(ty).is_some()
  {
    return false;
  }

  // No syntax for unnamed tables with a metatable
  if get_type_id::<MetatableType>(ty).is_some() {
    return false;
  }

  if let Some(ttv) = get_type_id::<TableType>(ty) {
    if ttv.name.is_some() {
      return true;
    }

    if ttv.synthetic_name.is_some() {
      return false;
    }
  }

  // We might still have a type with cycles or one that is too long, we'll check that later
  true
}
