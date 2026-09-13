use crate::{records::widen::Widen, type_aliases::type_pack_id::TypePackId};

impl Widen {
  pub fn is_dirty_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    false
  }
}
