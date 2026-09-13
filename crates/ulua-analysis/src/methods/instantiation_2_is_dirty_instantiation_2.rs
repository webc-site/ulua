use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{generic_type::GenericType, instantiation_2::Instantiation2},
  type_aliases::type_id::TypeId,
};

impl Instantiation2 {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let gt = get_type_id::<GenericType>(ty);
    if gt.is_none() {
      return false;
    }
    { self.generic_substitutions.find(&ty) }.is_some()
  }
}
