use crate::{records::apply_mapped_generics::ApplyMappedGenerics, type_aliases::type_id::TypeId};

impl ApplyMappedGenerics {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    unsafe { (*self.env).contains_mapped_type(ty) }
  }
}
