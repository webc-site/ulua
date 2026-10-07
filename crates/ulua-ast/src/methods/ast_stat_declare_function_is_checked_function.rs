//! `AstStatDeclareFunction::isCheckedFunction` (`Ast/src/Ast.cpp:1051`).
//! Hand-ported (the scheduler mutually false-blocks it against the identically
//! named `AstTypeFunction::isCheckedFunction` via a bare-name method edge).

use crate::{
  functions::has_attribute_in_array::has_attribute_in_array,
  records::{ast_attr::AstAttrType, ast_stat_declare_function::AstStatDeclareFunction},
};

impl AstStatDeclareFunction {
  pub fn is_checked_function(&self) -> bool {
    has_attribute_in_array(self.attributes, AstAttrType::Checked)
  }
}
