use core::ptr::NonNull;

use crate::{
  records::{replacer::Replacer, unifier_2::Unifier2},
  type_aliases::type_id::TypeId,
};

impl Unifier2 {
  pub fn instantiate_with_bound_types(&mut self, ty: TypeId) -> TypeId {
    let mut r = Replacer::new(
      self.arena.as_ptr(),
      NonNull::from(&mut self.generic_substitutions).as_ptr(),
      NonNull::from(&mut self.generic_pack_substitutions).as_ptr(),
    );
    if let Some(new_ty) = r.substitute_type_id(ty) {
      return new_ty;
    }
    ty
  }
}
