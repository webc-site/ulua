use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type, follow_type_pack, get_type, get_type_pack,
  },
  records::{
    any_type::AnyType, free_type::FreeType, free_type_pack::FreeTypePack,
    generic_type::GenericType, generic_type_pack::GenericTypePack, metatable_type::MetatableType,
    table_type::TableType,
  },
  type_aliases::{
    error_type::ErrorType, error_type_pack::ErrorTypePack, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

pub fn can_suggest_inferred_type(ty: TypeId) -> bool {
  let ty = follow_type::follow(ty);

  // No point in suggesting 'any', invalid to suggest others
  if get_type::get::<AnyType>(ty).is_some()
    || get_type::get::<ErrorType>(ty).is_some()
    || get_type::get::<GenericType>(ty).is_some()
    || get_type::get::<FreeType>(ty).is_some()
  {
    return false;
  }

  // No syntax for unnamed tables with a metatable
  if get_type::get::<MetatableType>(ty).is_some() {
    return false;
  }

  if let Some(ttv) = get_type::get::<TableType>(ty) {
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

/// C++ `static bool canSuggestInferredType(TypePackId ty)`.
pub(crate) fn can_suggest_inferred_type_type_pack_id(ty: TypePackId) -> bool {
  let ty = follow_type_pack::follow(ty);

  if get_type_pack::get::<ErrorTypePack>(ty).is_some()
    || get_type_pack::get::<GenericTypePack>(ty).is_some()
    || get_type_pack::get::<FreeTypePack>(ty).is_some()
  {
    return false;
  }

  let (head, _tail) = flatten_type_pack_id(ty);

  for head_ty in head {
    if !can_suggest_inferred_type(head_ty) {
      return false;
    }
  }

  true
}
