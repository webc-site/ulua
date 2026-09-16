use ulua_cli_lib::records::vfs_navigator::VfsNavigator;

use crate::type_aliases::{
  bool_check::BoolCheck, compile_options::CompileOptions, coverage::Coverage,
};

#[derive(Debug, Clone)]
pub struct ReplRequirer {
  pub(crate) copts: CompileOptions,
  pub(crate) coverage_active: BoolCheck,
  pub(crate) codegen_enable: BoolCheck,
  pub(crate) coverage_track: Coverage,
  pub(crate) counters_active: BoolCheck,
  pub(crate) counters_track: Coverage,
  pub(crate) vfs: VfsNavigator,
}
