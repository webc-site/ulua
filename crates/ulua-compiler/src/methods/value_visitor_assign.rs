use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
  visit::ast_expr_visit,
};

use crate::{enums::global::Global, records::value_visitor::ValueVisitor};

impl ValueVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn assign(&mut self, var: *mut AstExpr) {
    if var.is_null() {
      return;
    }

    let var_ptr = var as *mut AstNode;

    let local = unsafe { ast_node_as::<AstExprLocal>(var_ptr) };
    if !local.is_null() {
      let local_ptr = unsafe { (*local).local };
      // C++ `variables[lv->local].written = true`: operator[] creates the entry
      // if it doesn't exist, so use get_or_insert (find_mut would miss new keys,
      // e.g. a numeric for-loop variable assigned inside its body).
      self.variables.get_or_insert(local_ptr).written = true;
      return;
    }

    let global = unsafe { ast_node_as::<AstExprGlobal>(var_ptr) };
    if !global.is_null() {
      let name = unsafe { (*global).name };
      // C++ `globals[gv->name] = Global::Written`: operator[] overwrites; try_insert
      // would leave an existing Default entry unchanged.
      *self.globals.get_or_insert(name) = Global::Written;
      return;
    }

    // C++ `var->visit(this)`: track assignments inside complex lvalues such as
    // `t[function() t = nil end] = 5` by dispatching through the node walker.
    unsafe { ast_expr_visit(var, self) };
  }
}
