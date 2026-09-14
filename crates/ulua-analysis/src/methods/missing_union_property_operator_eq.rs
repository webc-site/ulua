use crate::records::missing_union_property::MissingUnionProperty;

impl MissingUnionProperty {
  #[inline]
  pub fn operator_eq(&self, rhs: &MissingUnionProperty) -> bool {
    self.missing == rhs.missing && self.r#type == rhs.r#type && self.key == rhs.key
  }
}

#[cfg(test)]
mod tests {
  use alloc::{string::ToString, vec, vec::Vec};

  use super::MissingUnionProperty;
  use crate::{records::r#type::Type, type_aliases::type_id::TypeId};

  /// TypeId 是 `*const Type`，测试用悬垂指针充当不同的类型标识。
  fn ty(v: u8) -> TypeId {
    v as *const Type
  }

  fn prop(ty_id: TypeId, missing: Vec<TypeId>, key: &str) -> MissingUnionProperty {
    MissingUnionProperty {
      r#type: ty_id,
      missing,
      key: key.to_string(),
    }
  }

  #[test]
  fn equal_props() {
    assert!(prop(ty(1), vec![ty(2), ty(3)], "x").operator_eq(&prop(
      ty(1),
      vec![ty(2), ty(3)],
      "x"
    )));
  }

  #[test]
  fn different_missing_element_not_equal() {
    assert!(!prop(ty(1), vec![ty(2), ty(3)], "x").operator_eq(&prop(
      ty(1),
      vec![ty(2), ty(4)],
      "x"
    )));
  }

  #[test]
  fn different_missing_length_not_equal() {
    assert!(!prop(ty(1), vec![ty(2)], "x").operator_eq(&prop(ty(1), vec![ty(2), ty(3)], "x")));
  }

  #[test]
  fn different_type_or_key_not_equal() {
    assert!(!prop(ty(1), vec![ty(2)], "x").operator_eq(&prop(ty(5), vec![ty(2)], "x")));
    assert!(!prop(ty(1), vec![ty(2)], "x").operator_eq(&prop(ty(1), vec![ty(2)], "y")));
  }
}
