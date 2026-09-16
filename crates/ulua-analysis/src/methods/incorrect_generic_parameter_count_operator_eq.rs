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

#[cfg(test)]
mod tests {
  use alloc::{string::ToString, vec};

  use super::IncorrectGenericParameterCount;
  use crate::{
    records::{generic_type_definition::GenericTypeDefinition, r#type::Type, type_fun::TypeFun},
    type_aliases::type_id::TypeId,
  };

  /// TypeId 是 `*const Type`，测试用悬垂指针充当不同的类型标识。
  fn ty(v: u8) -> TypeId {
    v as *const Type
  }

  fn type_fun(r#type: TypeId, params: &[u8]) -> TypeFun {
    TypeFun {
      type_params: params
        .iter()
        .map(|&p| GenericTypeDefinition {
          ty: ty(p),
          default_value: None,
        })
        .collect(),
      type_pack_params: vec![],
      r#type,
      definition_location: None,
    }
  }

  fn err(name: &str, r#type: TypeId, params: &[u8]) -> IncorrectGenericParameterCount {
    IncorrectGenericParameterCount {
      name: name.to_string(),
      type_fun: type_fun(r#type, params),
      actual_parameters: params.len(),
      actual_pack_parameters: 0,
    }
  }

  #[test]
  fn equal_values() {
    assert!(err("T", ty(1), &[2, 3]).operator_eq(&err("T", ty(1), &[2, 3])));
  }

  #[test]
  fn different_param_ty_not_equal() {
    assert!(!err("T", ty(1), &[2, 3]).operator_eq(&err("T", ty(1), &[2, 4])));
  }

  #[test]
  fn different_param_count_not_equal() {
    assert!(!err("T", ty(1), &[2]).operator_eq(&err("T", ty(1), &[2, 3])));
  }

  #[test]
  fn different_name_or_type_not_equal() {
    assert!(!err("T", ty(1), &[2]).operator_eq(&err("U", ty(1), &[2])));
    assert!(!err("T", ty(1), &[2]).operator_eq(&err("T", ty(5), &[2])));
  }
}
