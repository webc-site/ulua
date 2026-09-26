use core::ptr::null;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as_ptr},
};

use crate::records::{data_flow_graph::DataFlowGraph, refinement_key::RefinementKey};
pub fn match_is_instance_guard(call: &AstExprCall, dfg: &DataFlowGraph) -> *const RefinementKey {
  // Safety: call.func 是 parse arena 存活的表达式节点（或 null）；try_as_ptr 先判空
  // 再按 repr(C) 基类 class_index 甄别，未命中返回 None 且从不解引用，命中即返回
  // 完整存活 AstExprIndexName 的只读借用（arena 地址不移动、于 `call` 借用期内存活）。
  // ast_node_is 仅只读比较 class index。全程单线程只读。
  unsafe {
    let Some(index) = ast_node_try_as_ptr::<AstExprIndexName>(call.func) else {
      return null();
    };
    if index.op != b'.' {
      return null();
    }

    if index.index.value.is_null() || index.index.as_bytes() != b"isinstance" {
      return null();
    }

    if !ast_node_is_ptr::<AstExprGlobal>(index.expr) {
      return null();
    }

    let Some(&arg) = call.args.first() else {
      return null();
    };

    dfg.get_refinement_key(arg)
  }
}
