use std::panic::panic_any;

use ulua_analysis::records::data_flow_graph_builder::DataFlowGraphBuilder;
use ulua_ast::records::{parse_errors::ParseErrors, parse_options::ParseOptions, parser::Parser};

use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
impl DataFlowGraphFixture {
  pub fn dfg(&mut self, code: &str) {
    self.names.rebind_allocator(&mut self.allocator as *mut _);

    let result = Parser::parse(
      code,
      code.len(),
      &mut self.names,
      &mut self.allocator,
      ParseOptions::default(),
    );

    if !result.errors.is_empty() {
      panic_any(ParseErrors::new(result.errors));
    }

    self.module = result.root;
    self.graph = Some(unsafe {
      DataFlowGraphBuilder::build(
        self.module,
        &mut self.def_arena as *mut _,
        &mut self.key_arena as *mut _,
        &mut self.handle as *mut _,
      )
    });
  }
}
