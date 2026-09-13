use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::freeze::freeze,
  records::{frontend::Frontend, frontend_options::FrontendOptions},
};
use ulua_ast::enums::mode::Mode;
use ulua_common::FFlag;

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    let newly_initialized = self.frontend.is_none();

    if newly_initialized {
      let mode = if FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::Old
      } else {
        SolverMode::New
      };

      let options = FrontendOptions {
        retain_full_type_graphs: true,
        for_autocomplete: false,
        run_lint_checks: false,
        ..Default::default()
      };
      let frontend = Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
        mode,
        &mut self.file_resolver.base,
        &mut self.config_resolver.base,
        options,
      );

      self.frontend = Some(frontend);
      self.config_resolver.default_config.mode = Mode::Strict;
      self
        .config_resolver
        .default_config
        .enabled_lint
        .warning_mask = !0u64;
      self
        .config_resolver
        .default_config
        .parse_options
        .capture_comments = true;
    }

    let file_resolver = &mut self.file_resolver.base;
    let config_resolver = &mut self.config_resolver.base;
    let frontend = self.frontend.as_mut().unwrap();

    frontend.file_resolver = file_resolver;
    frontend.config_resolver = config_resolver;
    unsafe {
      frontend.wire_self_pointers();
    }

    if newly_initialized {
      freeze(frontend.globals.global_types_mut());
      freeze(frontend.globals_for_autocomplete.global_types_mut());
    }

    self.builtin_types = frontend.builtin_types;
    frontend
  }
}
