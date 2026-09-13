use alloc::vec::Vec;

use crate::{
  records::{lint_result::LintResult, type_error::TypeError},
  type_aliases::module_name_type::ModuleName,
};
#[derive(Debug, Clone, Default)]
pub struct CheckResult {
  pub errors: Vec<TypeError>,
  pub lint_result: LintResult,
  pub timeout_hits: Vec<ModuleName>,
}
