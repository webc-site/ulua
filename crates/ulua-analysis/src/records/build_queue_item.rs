use alloc::{sync::Arc, vec::Vec};

use ulua_config::records::config::Config;

use crate::{
  records::{
    frontend::FrontendStats, frontend_options::FrontendOptions,
    internal_compiler_error::InternalCompilerError, require_cycle::RequireCycle,
    source_module::SourceModule, source_node::SourceNode,
  },
  type_aliases::{
    module_name_type::ModuleName, module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr,
  },
};

#[derive(Debug, Clone)]
pub struct BuildQueueItem {
  pub name: ModuleName,
  pub human_readable_name: ModuleName,
  pub source_node: Arc<SourceNode>,
  pub source_module: Arc<SourceModule>,
  pub config: Config,
  pub environment_scope: ScopePtr,
  pub require_cycles: Vec<RequireCycle>,
  pub options: FrontendOptions,
  pub record_json_log: bool,
  pub reverse_deps: Vec<usize>,
  pub dirty_dependencies: i32,
  pub processing: bool,
  // Result
  // C++: `std::exception_ptr exception;` which here only ever holds a
  // `Luau::InternalCompilerError` (a recursion/internal compiler error). It is
  // used as a presence flag (`if (item.exception)`) and later rethrown in
  // `recordItemResult`. Modeled as an `Option` carrying the caught error.
  pub exception: Option<InternalCompilerError>,
  pub module: ModulePtr,
  pub stats: FrontendStats,
}
