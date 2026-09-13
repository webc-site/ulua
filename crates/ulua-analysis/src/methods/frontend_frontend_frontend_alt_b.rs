//! C++ `Frontend::Frontend(FileResolver*, ConfigResolver*, const
//! FrontendOptions& options)` (`Analysis/src/Frontend.cpp:448-459`).
use ulua_common::FFlag;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    config_resolver::ConfigResolver, file_resolver::FileResolver, frontend::Frontend,
    frontend_options::FrontendOptions,
  },
};

impl Frontend {
  /// Owned constructor delegating to the `SolverMode`-taking ctor. The only
  /// difference from C++ ctor #1 is the solver mode is derived from the
  /// `LuauSolverV2` fast flag: `useNewLuauSolver(FFlag::LuauSolverV2 ?
  /// SolverMode::New : SolverMode::Old)`.
  ///
  /// As with the other ctor, the returned value's self-referential pointers
  /// are wired by [`Frontend::wire_self_pointers`] after placement.
  pub fn frontend_file_resolver_config_resolver_frontend_options(
    file_resolver: *mut FileResolver,
    config_resolver: *mut ConfigResolver,
    options: &FrontendOptions,
  ) -> Self {
    let mode = if FFlag::LuauSolverV2.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    };

    Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
      mode,
      file_resolver,
      config_resolver,
      options.clone(),
    )
  }
}
