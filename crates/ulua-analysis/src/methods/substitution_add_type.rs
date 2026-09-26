use crate::{
  records::{substitution::Substitution, r#type::Type},
  type_aliases::type_id::TypeId,
};
impl Substitution {
  pub fn add_type<T>(&mut self, tv: T) -> TypeId
  where
    T: Into<Type>,
  {
    self.wired_arena_mut().add_tv(tv.into())
  }
}
