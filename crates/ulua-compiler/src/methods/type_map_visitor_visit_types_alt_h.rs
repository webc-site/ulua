use ulua_ast::{records::ast_stat_local::AstStatLocal, visit::ast_expr_visit};

use crate::records::type_map_visitor::TypeMapVisitor;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) fn visit_ast_stat_local(this: &mut TypeMapVisitor<'_>, node: *mut AstStatLocal) -> bool {
  unsafe {
    if node.is_null() {
      return false;
    }

    let node_ref = &*node;

    // Visit all value expressions
    for &expr_ptr in node_ref.values.as_slice() {
      if !expr_ptr.is_null() {
        ast_expr_visit(expr_ptr, this);
      }
    }

    // Propagate types from values to variables
    let vars = node_ref.vars.as_slice();
    let values = node_ref.values.as_slice();

    for (i, &var_ptr) in vars.iter().enumerate() {
      if var_ptr.is_null() {
        continue;
      }

      let var = &mut *var_ptr;

      // Propagate from the value that's being assigned
      // This simple propagation doesn't handle type packs in tail position
      if var.annotation.is_null() && i < values.len() {
        let value_ptr = values[i];
        if !value_ptr.is_null()
          && let Some(&type_ptr) = this.resolved_exprs.find(&value_ptr)
        {
          this.resolved_locals.try_insert(var_ptr, type_ptr);
        }
      }
    }
  }

  false
}

impl<'a> TypeMapVisitor<'a> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
    visit_ast_stat_local(self, node)
  }
}
