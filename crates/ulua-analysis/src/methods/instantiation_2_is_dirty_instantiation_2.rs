use crate::{
  functions::{get_type, get_type_pack},
  records::{
    generic_type::GenericType, generic_type_pack::GenericTypePack, instantiation_2::Instantiation2,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Instantiation2 {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let gt = get_type::get::<GenericType>(ty);
    if gt.is_none() {
      return false;
    }
    { self.generic_substitutions.find(&ty) }.is_some()
  }

  pub fn is_dirty_type_pack_id(&self, tp: TypePackId) -> bool {
    let generic_pack = get_type_pack::get::<GenericTypePack>(tp);
    generic_pack.is_some() && self.generic_pack_substitutions.find(&tp).is_some()
  }
}
