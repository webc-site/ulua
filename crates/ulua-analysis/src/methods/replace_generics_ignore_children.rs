use crate::{
  functions::{get_type_alt_j::get_type_id, get_type_utils::get_optional_ty},
  records::{
    extern_type::ExternType, function_type::FunctionType, replace_generics::ReplaceGenerics,
  },
  type_aliases::type_id::TypeId,
};

impl ReplaceGenerics {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    if let Some(ftv_ref) = get_type_id::<FunctionType>(ty) {
      if ftv_ref.has_no_free_or_generic_types {
        return true;
      }

      return (!self.generics.is_empty() || !self.generic_packs.is_empty())
        && (ftv_ref.generics == self.generics)
        && (ftv_ref.generic_packs == self.generic_packs);
    }

    let et = unsafe { get_optional_ty::<ExternType, TypeId>(Some(ty)) };
    !et.is_null()
  }
}
