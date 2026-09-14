use ulua_ast::records::ast_stat::AstStat;

use crate::{functions::always_terminates::always_terminates, records::compiler::Compiler};

impl Compiler {
  pub fn always_terminates(&self, node: *mut AstStat) -> bool {
    always_terminates(&self.constants, node)
  }
}
