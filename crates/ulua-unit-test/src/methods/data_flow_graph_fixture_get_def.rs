use ulua_analysis::type_aliases::def_id_def::DefId;
use ulua_ast::records::{ast_expr::AstExpr, ast_node::AstNode};

use crate::{
  functions::{nth::AstNodeClass, query::query},
  records::{data_flow_graph_fixture::DataFlowGraphFixture, nth::Nth},
};
impl DataFlowGraphFixture {
  pub fn get_def<T: AstNodeClass>(&mut self, nths: Vec<Nth>) -> DefId {
    let node = query::<T>(self.module as *mut AstNode, nths);
    ulua_common::LUAU_ASSERT!(!node.is_null());
    (*self
      .graph
      .as_ref()
      .expect("DataFlowGraphFixture.graph is None"))
    .get_def(node as *const AstExpr)
  }
}
