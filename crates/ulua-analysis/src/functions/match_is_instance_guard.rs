use core::{
  ffi::{CStr, c_char},
  ptr::null,
};

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName,
  },
  rtti::ast_node_as,
};

use crate::records::{data_flow_graph::DataFlowGraph, refinement_key::RefinementKey};
pub fn match_is_instance_guard(call: &AstExprCall, dfg: &DataFlowGraph) -> *const RefinementKey {
  unsafe {
    let index = ast_node_as::<AstExprIndexName>(call.func as *mut _);
    if index.is_null() || (*index).op != '.' as c_char {
      return null();
    }

    if (*index).index.value.is_null()
      || CStr::from_ptr((*index).index.value).to_bytes() != b"isinstance"
    {
      return null();
    }

    if ast_node_as::<AstExprGlobal>((*index).expr as *mut _).is_null() {
      return null();
    }

    if call.args.size < 1 {
      return null();
    }

    dfg.get_refinement_key(*call.args.data)
  }
}
