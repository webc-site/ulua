use crate::{records::widen::Widen, type_aliases::type_pack_id::TypePackId};

impl Widen {
  pub fn clean_type_pack_id(&mut self, _tp: TypePackId) -> TypePackId {
    panic!("Widen attempted to clean a dirty type pack?");
  }
}
