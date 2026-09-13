use core::ffi::c_void;

use ulua_cli_lib::records::vfs_navigator::VfsNavigator;

use crate::{
  records::repl_requirer::ReplRequirer,
  type_aliases::{bool_check::BoolCheck, compile_options::CompileOptions, coverage::Coverage},
};

impl ReplRequirer {
  pub fn repl_requirer_repl_requirer(
    copts: CompileOptions,
    coverage_active: BoolCheck,
    codegen_enabled: BoolCheck,
    coverage_track: Coverage,
    counters_active: BoolCheck,
    counters_track: Coverage,
    _in: *const c_void,
  ) -> Self {
    Self {
      copts,
      coverage_active,
      codegen_enable: codegen_enabled,
      coverage_track,
      counters_active,
      counters_track,
      vfs: VfsNavigator::default(),
    }
  }
}
