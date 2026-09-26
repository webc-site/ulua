use std::panic::panic_any;

use ulua_analysis::records::{arena_handle::Handle, data_flow_graph_builder::DataFlowGraphBuilder};
use ulua_ast::records::{parse_errors::ParseErrors, parse_options::ParseOptions, parser::Parser};

use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
impl DataFlowGraphFixture {
  pub fn dfg(&mut self, code: &str) {
    self.names.rebind_allocator(&mut self.allocator as *mut _);

    let result = Parser::parse(
      code,
      &mut self.names,
      &mut self.allocator,
      ParseOptions::default(),
    );

    if !result.errors.is_empty() {
      panic_any(ParseErrors::new(result.errors));
    }

    self.module = result.root;
    self.graph = Some(unsafe {
      // Safety: module 为解析成功（行 19 错误路径已 panic）时 Parser::parse 返回的存活 AstStatBlock，fixture allocator arena 块地址不动、比 graph 长寿；三个 arena 字段同属本结构不同字段、互不重叠，Handle 仅在 build 帧内使用，返回图不借用它们。
      DataFlowGraphBuilder::build(
        self.module,
        Handle::from_mut(&mut self.def_arena),
        Handle::from_mut(&mut self.key_arena),
        &mut self.handle as *mut _,
      )
    });
  }
}
