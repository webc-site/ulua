use ulua_ast::records::parse_result::ParseResult;

use crate::{records::source_module::SourceModule, type_aliases::module_ptr_module::ModulePtr};

#[derive(Debug, Clone)]
pub struct LoadDefinitionFileResult {
  pub success: bool,
  pub parse_result: ParseResult,
  pub source_module: SourceModule,
  // C++ `ModulePtr module;` — a `std::shared_ptr<Module>` that is `nullptr`
  // when parsing failed; `Arc<Module>` cannot be null, so it is `Option`.
  pub module: Option<ModulePtr>,
}
