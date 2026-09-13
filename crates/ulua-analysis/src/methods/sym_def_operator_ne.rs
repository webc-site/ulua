use crate::records::sym_def::SymDef;

impl SymDef {
  pub fn operator_ne(&self, other: &SymDef) -> bool {
    !self.operator_eq_sym_def(other)
  }
}
