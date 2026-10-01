use core::ptr::null;

use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr_call::AstExprCall};

use crate::records::{
  arena_handle::alias_opt, data_flow_graph::DataFlowGraph, refinement_key::RefinementKey,
};

pub fn match_is_instance_guard(call: &AstExprCall, dfg: &DataFlowGraph) -> *const RefinementKey {
  let Some(func) = alias_opt(call.func) else {
    return null();
  };

  if let AstExprRef::IndexName(index) = func.as_expr_ref() {
    if index.op != b'.' || index.index.as_bytes() != b"isinstance" {
      return null();
    }

    if !matches!(index.expr.as_expr_ref(), AstExprRef::Global(_)) {
      return null();
    }

    let Some(&arg) = call.args.first() else {
      return null();
    };

    dfg.get_refinement_key(arg)
  } else {
    null()
  }
}
