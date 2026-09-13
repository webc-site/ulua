use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    apply_type_function::ApplyTypeFunction, extern_type::ExternType, generic_type::GenericType,
  },
  type_aliases::type_id::TypeId,
};

impl ApplyTypeFunction {
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    if !get_type_id::<GenericType>(ty).is_none() {
      true
    } else {
      !get_type_id::<ExternType>(ty).is_none()
    }
  }
}
