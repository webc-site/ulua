use core::slice::from_raw_parts;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_local::AstLocal, ast_node::AstNode,
  },
  visit::ast_node_visit,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{constant::Constant, cost_visitor::CostVisitor};

/// 单个函数最多参与成本折扣的变量数（C++ `varCount < 7`）
pub(crate) const K_MAX_COST_VARS: usize = 7;
/// 变量折扣位宽
pub(crate) const K_VAR_DISCOUNT_BITS: usize = 8;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn model_cost(
  root: *mut AstNode,
  vars: *const *mut AstLocal,
  var_count: usize,
  builtins: &DenseHashMap<*mut AstExprCall, i32>,
  constants: &DenseHashMap<*mut AstExpr, Constant>,
) -> u64 {
  let mut visitor = CostVisitor::new(builtins, constants);

  if var_count > 0 {
    let vars = unsafe { from_raw_parts(vars, var_count.min(K_MAX_COST_VARS)) };
    for (i, &var_ptr) in vars.iter().enumerate() {
      *visitor.vars.get_or_insert(var_ptr) =
        0xffu64 << ((i * K_VAR_DISCOUNT_BITS + K_VAR_DISCOUNT_BITS) as u32);
    }
  }

  unsafe {
    ast_node_visit(root, &mut visitor);
  }

  visitor.result.model
}
