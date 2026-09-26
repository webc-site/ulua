use alloc::{string::String, sync::Arc, vec::Vec};

use ulua_config::records::config::Config;

use crate::{
  records::{
    frontend::FrontendStats, frontend_options::FrontendOptions, require_cycle::RequireCycle,
    source_module::SourceModule, source_node::SourceNode,
  },
  type_aliases::{
    module_name_type::ModuleName, module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr,
  },
};

#[derive(Debug, Clone)]
pub struct BuildQueueItem {
  pub name: ModuleName,
  pub human_readable_name: String,
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
  // cpp `std::exception_ptr exception` 的对应物已删除：本端口把 InternalCompilerError
  // 建模为 panic（见 perform_queue_item_task），任务成功路径上该字段恒为 None，
  // 失败直接 unwind 出 check_queued_modules，无需 presence flag。
  pub module: ModulePtr,
  pub stats: FrontendStats,
}
