use ulua_analysis::type_aliases::def_id_def::DefId;
use ulua_ast::{records::ast_expr::AstExpr, rtti::AstNodeClass};

use crate::{
  functions::query::query,
  records::{data_flow_graph_fixture::DataFlowGraphFixture, nth::Nth},
};
impl DataFlowGraphFixture {
  pub fn get_def<T: AstNodeClass>(&mut self, nths: Vec<Nth>) -> DefId {
    // `query` 吃 `impl AstNodePtr`：根块指针免手写 `as *mut AstNode` 上转。
    let node = query::<T>(self.module, nths);
    ulua_common::LUAU_ASSERT!(!node.is_null());
    (*self
      .graph
      .as_ref()
      .expect("DataFlowGraphFixture.graph is None"))
    .get_def(node.cast_const().cast::<AstExpr>())
  }
}
