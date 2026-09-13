use crate::{
  records::{free_type_pack::FreeTypePack, internal_type_finder::InternalTypeFinder},
  type_aliases::type_pack_id::TypePackId,
};

impl InternalTypeFinder {
  pub fn visit_type_pack_id_free_type_pack(
    &mut self,
    _tp: TypePackId,
    _ftp: &FreeTypePack,
  ) -> bool {
    false
  }
}
