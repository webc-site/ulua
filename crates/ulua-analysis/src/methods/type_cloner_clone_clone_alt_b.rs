use crate::{records::type_cloner::TypeCloner, type_aliases::type_pack_id::TypePackId};

impl TypeCloner {
  pub fn clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    self.shallow_clone_type_pack_id(tp);
    self.run();
    if self.has_exceeded_iteration_limit() {
      let error = unsafe { (*self.builtin_types).error_type_pack };
      unsafe { (*self.packs).insert(tp, error) };
      return error;
    }
    self
      .find_type_pack_id(tp)
      .unwrap_or(unsafe { (*self.builtin_types).error_type_pack })
  }
}
