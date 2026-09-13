use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type::GenericType, scope::Scope},
};

impl GenericType {
  pub fn generic_type_scope_polarity(scope: *mut Scope, polarity: Polarity) -> Self {
    GenericType {
      index: fresh_index(),
      level: Default::default(),
      scope,
      name: Default::default(),
      explicit_name: false,
      polarity,
    }
  }
}
