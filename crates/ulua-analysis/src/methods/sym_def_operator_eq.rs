use crate::records::sym_def::SymDef;

impl SymDef {
  pub fn operator_eq(&self, other: &SymDef) -> bool {
    self.sym.operator_eq_symbol(&other.sym) && self.version == other.version
  }
}
