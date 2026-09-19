use crate::{
  records::{generic_type_pack::GenericTypePack, type_cacher::TypeCacher},
  type_aliases::type_pack_id::TypePackId,
};

impl TypeCacher {
  pub fn visit_type_pack_id_generic_type_pack(
    &mut self,
    _tp: TypePackId,
    _gtp: &GenericTypePack,
  ) -> bool {
    true
  }
}
