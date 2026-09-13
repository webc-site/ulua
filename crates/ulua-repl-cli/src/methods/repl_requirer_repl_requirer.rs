use crate::{
  records::repl_requirer::ReplRequirer,
  type_aliases::{bool_check::BoolCheck, compile_options::CompileOptions, coverage::Coverage},
};

pub fn repl_requirer_repl_requirer(
  copts: CompileOptions,
  coverage_active: BoolCheck,
  codegen_enabled: BoolCheck,
  coverage_track: Coverage,
  counters_active: BoolCheck,
  counters_track: Coverage,
) -> ReplRequirer {
  ReplRequirer {
    copts,
    coverage_active,
    codegen_enable: codegen_enabled,
    coverage_track,
    counters_active,
    counters_track,
    vfs: Default::default(),
  }
}
