use crate::{
  records::apply_mapped_generics::ApplyMappedGenerics,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ApplyMappedGenerics {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    unsafe { (*self.env).contains_mapped_type(ty) }
  }

  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    unsafe { (*self.env).contains_mapped_pack(tp) }
  }
}
