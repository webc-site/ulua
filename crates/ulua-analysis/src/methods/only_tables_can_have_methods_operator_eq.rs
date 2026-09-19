use crate::records::only_tables_can_have_methods::OnlyTablesCanHaveMethods;

impl OnlyTablesCanHaveMethods {
  #[inline]
  pub fn operator_eq(&self, rhs: &OnlyTablesCanHaveMethods) -> bool {
    self.table_type == rhs.table_type
  }
}
