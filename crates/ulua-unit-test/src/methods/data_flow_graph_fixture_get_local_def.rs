use ulua_analysis::type_aliases::def_id_def::DefId;
use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::{
  functions::{nth::nth_t, query::query},
  records::data_flow_graph_fixture::DataFlowGraphFixture,
};

impl DataFlowGraphFixture {
  pub fn get_local_def(&self, stat_n: i32, var_index: usize) -> DefId {
    // `query` 吃 `impl AstNodePtr`：根块指针免手写 `as *mut AstNode` 上转。
    let local_stat = query::<AstStatLocal>(self.module, vec![nth_t::<AstStatLocal>(stat_n)]);
    assert!(!local_stat.is_null());

    let local = unsafe {
      // Safety: local_stat 行 13 断言非空且经 query::<AstStatLocal> 判型命中（存活 AstStatLocal）；vars.as_slice() 在界内，读元素指针拷贝。
      (&*local_stat).vars.as_slice()[var_index]
    };
    self
      .graph
      .as_ref()
      .expect("DataFlowGraphFixture.graph is None")
      .get_def_ast_local(local)
  }
}
