use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type::GenericType, scope::Scope},
  type_aliases::name_type::Name,
};
impl GenericType {
  pub fn generic_type_scope_name_polarity(
    scope: *mut Scope,
    name: Name,
    polarity: Polarity,
  ) -> Self {
    GenericType {
      index: fresh_index(),
      level: Default::default(),
      scope,
      name,
      explicit_name: true,
      polarity,
    }
  }
}
