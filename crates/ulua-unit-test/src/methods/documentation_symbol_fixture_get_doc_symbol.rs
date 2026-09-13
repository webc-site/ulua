use ulua_analysis::{
  functions::get_documentation_symbol_at_position::get_documentation_symbol_at_position,
  type_aliases::documentation_symbol::DocumentationSymbol,
};
use ulua_ast::records::position::Position;

use crate::records::documentation_symbol_fixture::DocumentationSymbolFixture;

impl DocumentationSymbolFixture {
  pub fn get_doc_symbol(
    &mut self,
    source: &str,
    position: Position,
  ) -> Option<DocumentationSymbol> {
    self.base.get_frontend();
    self
      .base
      .base
      .check_string_optional_frontend_options(source, None);

    let source_module = self.base.base.get_main_source_module();
    let module = self.base.base.get_main_module(false);

    unsafe { get_documentation_symbol_at_position(&*source_module, &*module, position) }
  }
}
