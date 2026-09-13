use crate::records::pack_where_clause_needed::PackWhereClauseNeeded;

impl PackWhereClauseNeeded {
  #[inline]
  pub fn operator_eq(&self, rhs: &PackWhereClauseNeeded) -> bool {
    self.tp == rhs.tp
  }
}
