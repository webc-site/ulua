use alloc::{string::String, vec::Vec};
use core::ptr::null_mut;

use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal};
#[derive(Debug, Clone)]
pub struct Global {
  pub(crate) first_ref: *mut AstExprGlobal,
  pub(crate) function_ref: Vec<*mut AstExprFunction>,
  pub(crate) assigned: bool,
  pub(crate) builtin: bool,
  pub(crate) defined_in_module_scope: bool,
  pub(crate) defined_as_function: bool,
  pub(crate) read_before_written: bool,
  pub(crate) deprecated: Option<String>,
}

impl Default for Global {
  fn default() -> Self {
    Self {
      first_ref: null_mut(),
      function_ref: Vec::new(),
      assigned: false,
      builtin: false,
      defined_in_module_scope: false,
      defined_as_function: false,
      read_before_written: false,
      deprecated: None,
    }
  }
}

impl Global {
  pub fn first_ref(&self) -> *mut AstExprGlobal {
    self.first_ref
  }

  pub fn function_ref(&self) -> &[*mut AstExprFunction] {
    &self.function_ref
  }

  pub fn assigned(&self) -> bool {
    self.assigned
  }

  pub fn builtin(&self) -> bool {
    self.builtin
  }

  pub fn defined_in_module_scope(&self) -> bool {
    self.defined_in_module_scope
  }

  pub fn defined_as_function(&self) -> bool {
    self.defined_as_function
  }

  pub fn read_before_written(&self) -> bool {
    self.read_before_written
  }

  pub fn deprecated(&self) -> Option<&str> {
    self.deprecated.as_deref()
  }
}
