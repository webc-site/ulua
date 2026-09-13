use ulua_analysis::records::check_result::CheckResult;
use ulua_common::FFlag;

use crate::{
  records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};
impl NonStrictTypeCheckerFixture {
  pub fn check_non_strict_module(&mut self, module_name: &str) -> CheckResult {
    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    self.get_frontend();
    let definitions = self.definitions.clone();
    let res = self.base.load_definition(&definitions, false);
    assert!(res.success);
    self
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from(module_name), None)
  }
}
