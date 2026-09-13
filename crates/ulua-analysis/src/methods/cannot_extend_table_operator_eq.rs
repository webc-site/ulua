use crate::records::cannot_extend_table::CannotExtendTable;

impl CannotExtendTable {
  #[inline]
  pub fn operator_eq(&self, rhs: &CannotExtendTable) -> bool {
    self.table_type == rhs.table_type && self.prop == rhs.prop && self.context == rhs.context
  }
}
