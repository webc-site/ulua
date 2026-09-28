use crate::{records::type_fun::TypeFun, type_aliases::name_type::Name};

#[derive(Debug, Clone, PartialEq)]
pub struct IncorrectGenericParameterCount {
  pub(crate) name: Name,
  pub(crate) type_fun: TypeFun,
  pub(crate) actual_parameters: usize,
  pub(crate) actual_pack_parameters: usize,
}

impl IncorrectGenericParameterCount {
  pub fn new(
    name: impl Into<Name>,
    type_fun: TypeFun,
    actual_parameters: usize,
    actual_pack_parameters: usize,
  ) -> Self {
    Self {
      name: name.into(),
      type_fun,
      actual_parameters,
      actual_pack_parameters,
    }
  }
}
