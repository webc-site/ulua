use crate::records::not_a_table::NotATable;

impl NotATable {
  #[inline]
  pub fn operator_eq(&self, rhs: &NotATable) -> bool {
    self.ty == rhs.ty
  }
}
