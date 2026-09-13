use crate::records::lint_global_local::LintGlobalLocal;
#[derive(Debug, Clone)]
pub struct HoldConditionalExecution {
  pub reset_to_false: bool,
  pub p: *mut LintGlobalLocal,
}
