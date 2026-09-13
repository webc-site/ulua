use crate::records::constant_key::ConstantKey;

impl ConstantKey {
  #[inline]
  pub const fn ir_builder_constant_key_operator_eq(&self, key: &ConstantKey) -> bool {
    self.kind as i32 == key.kind as i32 && self.value == key.value
  }
}
