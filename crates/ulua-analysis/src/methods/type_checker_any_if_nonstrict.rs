use crate::{records::type_checker::TypeChecker, type_aliases::type_id::TypeId};

impl TypeChecker {
  pub fn any_if_nonstrict(&mut self, ty: TypeId) -> TypeId {
    if self.is_nonstrict_mode() {
      self.any_type
    } else {
      ty
    }
  }
}
