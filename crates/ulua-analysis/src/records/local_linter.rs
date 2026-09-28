/// C++ `LintLocalHygiene::Local` (`Analysis/src/Linter.cpp:716`).
///
/// ```cpp
/// struct Local
/// {
///     AstNode* defined = nullptr;
///     bool function;
///     bool import;
///     bool used;
///     bool arg;
/// };
/// ```
use core::ptr::null_mut;

use ulua_ast::records::ast_node::AstNode;
#[derive(Debug, Clone)]
pub struct Local {
  pub(crate) defined: *mut AstNode,
  pub(crate) function: bool,
  pub(crate) import: bool,
  pub(crate) used: bool,
  pub(crate) arg: bool,
}

impl Default for Local {
  fn default() -> Self {
    Self {
      defined: null_mut(),
      function: false,
      import: false,
      used: false,
      arg: false,
    }
  }
}
