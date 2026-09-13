use crate::{
  records::apply_mapped_generics::ApplyMappedGenerics, type_aliases::type_pack_id::TypePackId,
};

impl ApplyMappedGenerics {
  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    unsafe { (*self.env).contains_mapped_pack(tp) }
  }
}
