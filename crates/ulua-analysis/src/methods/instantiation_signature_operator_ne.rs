use crate::records::instantiation_signature::InstantiationSignature;

impl InstantiationSignature {
  pub fn operator_ne(&self, rhs: &InstantiationSignature) -> bool {
    !self.operator_eq(rhs)
  }
}
