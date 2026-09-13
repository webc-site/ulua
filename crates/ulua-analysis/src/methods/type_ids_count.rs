use crate::{
  functions::follow_type::follow_type_id, records::type_ids::TypeIds, type_aliases::type_id::TypeId,
};

impl TypeIds {
  /// C++ `size_t count(TypeId ty) const`.
  pub fn count(&self, ty: TypeId) -> usize {
    let ty = follow_type_id(ty);
    match self.types.find(&ty) {
      Some(entry) if *entry => 1,
      _ => 0,
    }
  }
}
