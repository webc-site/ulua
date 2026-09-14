use ulua_ast::records::{ast_array::AstArray, ast_generic_type_pack::AstGenericTypePack};

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  pub fn visit_generic_packs(&mut self, g: AstArray<*mut AstGenericTypePack>) {
    for &generic in g.as_slice() {
      if !generic.is_null() && !unsafe { (*generic).default_value }.is_null() {
        self.visit_type_pack_ast_type_pack(unsafe { (*generic).default_value });
      }
    }
  }
}
