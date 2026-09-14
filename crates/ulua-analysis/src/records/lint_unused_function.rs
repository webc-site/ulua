use core::{
  ffi::{CStr, c_char, c_void},
  ptr::null_mut,
};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_name::AstName, ast_node::AstNode,
    ast_stat_function::AstStatFunction, ast_visitor::AstVisitor, location::Location,
  },
  rtti::ast_node_as,
  visit::{ast_expr_visit, ast_stat_visit},
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};
use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_context::LintContext};
#[derive(Debug, Clone, Default)]
pub struct Global {
  pub(crate) location: Location,
  pub(crate) function: bool,
  pub(crate) used: bool,
}

impl DenseDefault for Global {
  fn dense_default() -> Self {
    Self::default()
  }
}

#[derive(Debug, Clone)]
pub struct LintUnusedFunction {
  pub(crate) context: *mut LintContext,
  pub(crate) globals: DenseHashMap<AstName, Global>,
}

impl LintUnusedFunction {
  pub fn new() -> Self {
    Self {
      context: null_mut(),
      globals: DenseHashMap::new(AstName::new()),
    }
  }

  pub fn process(&mut self, context: &mut LintContext) {
    self.context = context as *mut LintContext;
    // SAFETY: The visitor pattern requires a valid AST root.
    // The context is guaranteed to be valid for the duration of the visit.
    unsafe {
      let root = (*self.context).root;
      ast_stat_visit(root, self);
    }
    self.report();
  }

  pub fn report(&mut self) {
    for (name, global) in self.globals.iter() {
      if global.function && !global.used {
        let c_str = name.value;
        if !c_str.is_null() {
          let first_char = unsafe { *c_str };
          if first_char != b'_' as c_char {
            let name = unsafe { CStr::from_ptr(c_str).to_string_lossy() };
            emit_warning(
              unsafe { &mut *self.context },
              Code::FunctionUnused,
              global.location,
              format_args!(
                "Function '{}' is never used; prefix with '_' to silence",
                name
              ),
            );
          }
        }
      }
    }
  }

  pub fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatFunction;
    unsafe {
      let name_expr = (*node).name;
      let expr = ast_node_as::<AstExprGlobal>(name_expr as *mut AstNode);
      if !expr.is_null() {
        let g = self.globals.get_or_insert((*expr).name);
        g.function = true;
        g.location = (*expr).base.base.location;
        ast_expr_visit((*node).func as *mut AstExpr, self);
        return false;
      }
    }
    true
  }

  pub fn visit_expr_global(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstExprGlobal;
    unsafe {
      let g = self.globals.get_or_insert((*node).name);
      g.used = true;
    }
    true
  }
}

impl Default for LintUnusedFunction {
  fn default() -> Self {
    Self::new()
  }
}

impl AstVisitor for LintUnusedFunction {
  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    self.visit_stat_function(node)
  }

  fn visit_expr_global(&mut self, node: *mut c_void) -> bool {
    self.visit_expr_global(node)
  }

  fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
