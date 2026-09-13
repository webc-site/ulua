use crate::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;

impl IncorrectGenericParameterCount {
  #[inline]
  pub fn operator_eq(&self, rhs: &IncorrectGenericParameterCount) -> bool {
    if self.name != rhs.name {
      return false;
    }

    if self.type_fun.r#type != rhs.type_fun.r#type {
      return false;
    }

    if self.type_fun.type_params.len() != rhs.type_fun.type_params.len() {
      return false;
    }

    if self.type_fun.type_pack_params.len() != rhs.type_fun.type_pack_params.len() {
      return false;
    }

    for i in 0..self.type_fun.type_params.len() {
      if self.type_fun.type_params[i].ty != rhs.type_fun.type_params[i].ty {
        return false;
      }
    }

    for i in 0..self.type_fun.type_pack_params.len() {
      if self.type_fun.type_pack_params[i].tp != rhs.type_fun.type_pack_params[i].tp {
        return false;
      }
    }

    true
  }
}
