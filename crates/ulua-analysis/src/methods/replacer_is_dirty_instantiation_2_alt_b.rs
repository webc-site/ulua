use crate::{records::replacer::Replacer, type_aliases::type_pack_id::TypePackId};

impl Replacer {
  pub fn is_dirty_type_pack_id(&self, tp: TypePackId) -> bool {
    unsafe { (*self.replacement_packs).find(&tp).is_some() }
  }
}
