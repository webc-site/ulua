use crate::{
  records::type_cacher::TypeCacher,
  type_aliases::{error_type_pack::ErrorTypePack, type_pack_id::TypePackId},
};

impl TypeCacher {
  pub fn visit_type_pack_id_error_type_pack(
    &mut self,
    _tp: TypePackId,
    _etp: &ErrorTypePack,
  ) -> bool {
    true
  }
}
