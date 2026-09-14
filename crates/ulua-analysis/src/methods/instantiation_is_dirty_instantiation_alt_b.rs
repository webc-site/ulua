use crate::{records::instantiation::Instantiation, type_aliases::type_pack_id::TypePackId};

impl Instantiation {
  pub fn is_dirty_type_pack_id(&self, _tp: TypePackId) -> bool {
    false
  }
}
