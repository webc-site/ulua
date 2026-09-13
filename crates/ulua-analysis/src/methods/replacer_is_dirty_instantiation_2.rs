use crate::{records::replacer::Replacer, type_aliases::type_id::TypeId};

impl Replacer {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    unsafe { (*self.replacements).find(&ty).is_some() }
  }
}
