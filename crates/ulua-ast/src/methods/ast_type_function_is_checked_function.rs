//! `AstTypeFunction::isCheckedFunction` (`Ast/src/Ast.cpp:1218`).
//! Hand-ported (mutually false-blocked against the identically named
//! `AstStatDeclareFunction::isCheckedFunction` via a bare-name method edge).

use crate::{
  functions::has_attribute_in_array::has_attribute_in_array,
  records::{ast_attr::AstAttrType, ast_type_function::AstTypeFunction},
};

impl AstTypeFunction {
  pub fn is_checked_function(&self) -> bool {
    has_attribute_in_array(self.attributes, AstAttrType::Checked)
  }
}
