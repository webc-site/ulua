use crate::{
  functions::{get_type_alt_j::get_type_id, get_type_utils::get_optional_ty},
  records::{extern_type::ExternType, function_type::FunctionType, instantiation::Instantiation},
  type_aliases::type_id::TypeId,
};

impl Instantiation {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    let ft = get_type_id::<FunctionType>(ty);
    if !ft.is_none() {
      return true;
    }

    let et = unsafe { get_optional_ty::<ExternType, TypeId>(Some(ty)) };
    !et.is_null()
  }
}
