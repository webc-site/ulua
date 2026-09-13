use crate::records::instantiation_signature::InstantiationSignature;

impl InstantiationSignature {
  pub fn operator_eq(&self, rhs: &InstantiationSignature) -> bool {
    self.fn_sig.operator_eq(&rhs.fn_sig)
      && self.arguments == rhs.arguments
      && self.pack_arguments == rhs.pack_arguments
  }
}
