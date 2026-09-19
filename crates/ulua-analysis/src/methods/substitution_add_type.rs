use crate::{
  records::{substitution::Substitution, r#type::Type},
  type_aliases::type_id::TypeId,
};
impl Substitution {
  pub fn add_type<T>(&mut self, tv: T) -> TypeId
  where
    T: Into<Type>,
  {
    unsafe { (*self.arena).add_tv(tv.into()) }
  }
}
