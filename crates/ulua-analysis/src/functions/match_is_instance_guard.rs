use core::ptr::null;

use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr_call::AstExprCall};

use crate::records::{data_flow_graph::DataFlowGraph, refinement_key::RefinementKey};

pub fn match_is_instance_guard(call: &AstExprCall, dfg: &DataFlowGraph) -> *const RefinementKey {
  // Safety: call.func 是 parse arena 存活的表达式节点（或 null）；
  // as_ref 先判空，as_expr_ref 基于 repr(C) 基类 class_index 安全模式匹配具体枚举。全程单线程只读。
  let Some(func) = (unsafe { call.func.as_ref() }) else {
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
