use crate::records::ast_name::AstName;

impl AstName {
  pub fn operator_lt(&self, rhs: &AstName) -> bool {
    self < rhs
  }
}
