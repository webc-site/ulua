#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Stats {
  pub(crate) files: usize,
  pub(crate) lines: usize,
  pub(crate) files_strict: usize,
  pub(crate) files_nonstrict: usize,
  pub(crate) types_allocated: usize,
  pub(crate) type_packs_allocated: usize,
  pub(crate) bool_singletons_minted: usize,
  pub(crate) str_singletons_minted: usize,
  pub(crate) unique_str_singletons_minted: usize,
  pub(crate) time_read: f64,
  pub(crate) time_parse: f64,
  pub(crate) time_check: f64,
  pub(crate) time_lint: f64,
  pub(crate) dynamic_constraints_created: usize,
}
