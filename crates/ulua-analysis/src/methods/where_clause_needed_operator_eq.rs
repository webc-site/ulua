use crate::records::where_clause_needed::WhereClauseNeeded;

impl WhereClauseNeeded {
  #[inline]
  pub fn operator_eq(&self, rhs: &WhereClauseNeeded) -> bool {
    self.ty == rhs.ty
  }
}
