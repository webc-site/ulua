use alloc::sync::Arc;

use ulua_config::records::lint_options::LintOptions;

use crate::records::{
  frontend_cancellation_token::FrontendCancellationToken, module::Module,
  source_module::SourceModule,
};

/// Port of `Luau::FrontendOptions` from `Analysis/include/Luau/Frontend.h`.
#[derive(Debug, Clone, Default)]
pub struct FrontendOptions {
  pub retain_full_type_graphs: bool,
  pub for_autocomplete: bool,
  pub run_lint_checks: bool,
  pub randomize_constraint_resolution_seed: Option<u32>,
  pub enabled_lint_warnings: Option<LintOptions>,
  pub cancellation_token: Option<Arc<FrontendCancellationToken>>,
  pub module_time_limit_sec: Option<f64>,
  pub apply_internal_limit_scaling: bool,
  pub custom_module_check: Option<fn(&SourceModule, &Module)>,
  pub collect_type_allocation_stats: bool,
}
