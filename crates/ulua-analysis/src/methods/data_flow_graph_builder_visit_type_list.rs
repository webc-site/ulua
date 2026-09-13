use ulua_ast::records::ast_type_list::AstTypeList;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  pub fn visit_type_list(&mut self, l: AstTypeList) {
    for &t in l.types.as_slice() {
      self.visit_type_ast_type(t);
    }

    if !l.tail_type.is_null() {
      self.visit_type_pack_ast_type_pack(l.tail_type);
    }
  }
}
