use crate::{records::type_cloner::TypeCloner, type_aliases::type_id::TypeId};

impl TypeCloner {
  pub fn clone_type_id(&mut self, ty: TypeId) -> TypeId {
    self.shallow_clone_type_id(ty);
    self.run();
    if self.has_exceeded_iteration_limit() {
      let error = unsafe { (*self.builtin_types).error_type };
      unsafe { (*self.types).insert(ty, error) };
      return error;
    }
    self
      .find_type_id(ty)
      .unwrap_or(unsafe { (*self.builtin_types).error_type })
  }
}
