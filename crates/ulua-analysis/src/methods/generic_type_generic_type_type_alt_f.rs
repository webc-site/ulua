use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type::GenericType, scope::Scope, type_level::TypeLevel},
  type_aliases::name_type::Name,
};

impl GenericType {
  pub fn generic_type_scope_name(scope: *mut Scope, name: &Name) -> Self {
    GenericType {
      index: fresh_index(),
      level: TypeLevel::default(),
      scope,
      name: name.clone(),
      explicit_name: true,
      polarity: Polarity::Unknown,
    }
  }
}
