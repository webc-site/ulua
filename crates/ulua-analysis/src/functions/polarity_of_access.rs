use ulua_ast::enums::ast_table_access::AstTableAccess;

use crate::{enums::polarity::Polarity, functions::invert_polarity::invert};

pub fn polarity_of_access(access: AstTableAccess, p: Polarity) -> Polarity {
  match access {
    AstTableAccess::Read => p,
    AstTableAccess::Write => invert(p),
    AstTableAccess::ReadWrite => Polarity::Mixed,
  }
}
