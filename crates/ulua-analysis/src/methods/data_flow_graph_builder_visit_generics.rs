use ulua_ast::records::{ast_array::AstArray, ast_generic_type::AstGenericType};

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  pub fn visit_generics(&mut self, g: AstArray<*mut AstGenericType>) {
    for &generic in g.as_slice() {
      if generic.is_null() {
        continue;
      }

      let default_value = unsafe { (*generic).default_value };
      if !default_value.is_null() {
        self.visit_type_ast_type(default_value);
      }
    }
  }
}
