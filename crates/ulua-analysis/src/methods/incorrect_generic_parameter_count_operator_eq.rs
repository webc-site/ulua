use crate::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;

impl IncorrectGenericParameterCount {
  #[inline]
  pub fn operator_eq(&self, rhs: &IncorrectGenericParameterCount) -> bool {
    self.name == rhs.name
      && self.type_fun.r#type == rhs.type_fun.r#type
      && self.type_fun.type_params.len() == rhs.type_fun.type_params.len()
      && self.type_fun.type_pack_params.len() == rhs.type_fun.type_pack_params.len()
      // 逐元素比较：类型参数比 `.ty`，类型包参数比 `.tp`
      && self
        .type_fun
        .type_params
        .iter()
        .zip(&rhs.type_fun.type_params)
        .all(|(l, r)| l.ty == r.ty)
      && self
        .type_fun
        .type_pack_params
        .iter()
        .zip(&rhs.type_fun.type_pack_params)
        .all(|(l, r)| l.tp == r.tp)
  }
}
