use crate::{
  functions::{
    can_suggest_inferred_type_autocomplete_core::can_suggest_inferred_type,
    flatten_type_pack::flatten_type_pack_id, follow_type_pack::follow_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  records::{free_type_pack::FreeTypePack, generic_type_pack::GenericTypePack},
  type_aliases::{error_type_pack::ErrorTypePack, type_pack_id::TypePackId},
};

/// C++ `static bool canSuggestInferredType(TypePackId ty)`.
pub(crate) fn can_suggest_inferred_type_type_pack_id(ty: TypePackId) -> bool {
  unsafe {
    let ty = follow_type_pack_id(ty);

    if !get_type_pack_id::<ErrorTypePack>(ty).is_none()
      || !get_type_pack_id::<GenericTypePack>(ty).is_none()
      || !get_type_pack_id::<FreeTypePack>(ty).is_none()
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
}
