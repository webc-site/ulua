use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{generic_type_pack::GenericTypePack, instantiation_2::Instantiation2},
  type_aliases::type_pack_id::TypePackId,
};

impl Instantiation2 {
  pub fn is_dirty_type_pack_id(&self, tp: TypePackId) -> bool {
    let generic_pack = get_type_pack_id::<GenericTypePack>(tp);
    !generic_pack.is_none() && self.generic_pack_substitutions.find(&tp).is_some()
  }
}
