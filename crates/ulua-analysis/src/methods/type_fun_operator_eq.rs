use crate::records::type_fun::TypeFun;

impl TypeFun {
  #[inline]
  pub fn operator_eq(&self, rhs: &TypeFun) -> bool {
    self.r#type == rhs.r#type
      && self.type_params == rhs.type_params
      && self.type_pack_params == rhs.type_pack_params
  }
}
