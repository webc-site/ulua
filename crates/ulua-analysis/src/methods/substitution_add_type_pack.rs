use crate::{
  records::{substitution::Substitution, type_pack_var::TypePackVar},
  type_aliases::type_pack_id::TypePackId,
};

impl Substitution {
  pub fn add_type_pack<T>(&mut self, tp: T) -> TypePackId
  where
    T: Into<TypePackVar>,
  {
    unsafe { (*self.arena).add_type_pack_t(tp.into()) }
  }
}
