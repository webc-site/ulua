use crate::{records::subtyping_environment::SubtypingEnvironment, type_aliases::type_id::TypeId};

impl SubtypingEnvironment {
  pub fn try_find_substitution(&self, ty: TypeId) -> Option<TypeId> {
    if let Some(it) = self.substitutions.find(&ty) {
      return Some(*it);
    }

    if !self.parent.is_null() {
      return unsafe { (*self.parent).try_find_substitution(ty) };
    }

    None
  }
}
