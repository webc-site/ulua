use crate::{
  records::{def::Def, non_strict_context::NonStrictContext},
  type_aliases::type_id::TypeId,
};

impl NonStrictContext {
  pub fn find_def(&self, d: *const Def) -> Option<TypeId> {
    self.context.get(&d).copied()
  }
}
