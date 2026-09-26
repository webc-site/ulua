use ulua_analysis::functions::get_documentation_symbol_at_position::get_documentation_symbol_at_position;
use ulua_ast::records::position::Position;

use crate::records::documentation_symbol_fixture::DocumentationSymbolFixture;

impl DocumentationSymbolFixture {
  pub fn get_doc_symbol(&mut self, source: &str, position: Position) -> Option<String> {
    self.base.get_frontend();
    self
      .base
      .base
      .check_string_optional_frontend_options(source, None);

    let source_module = self
      .base
      .base
      .get_main_source_module()
      .expect("main source module must exist after check");
    let module = self.base.base.get_main_module(false);

    // Safety: module 为 check 后 resolver 容器保有的存活 Module（&* 物化只读
    // 借用，对应 cpp Module& 传参），查询只读、借用随调用结束；
    // source_module 为 Handle 交付的共享只读借用（契约见 arena_handle 模块）。
    unsafe { get_documentation_symbol_at_position(source_module.get(), &*module, position) }
  }
}
