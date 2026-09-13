use ulua_analysis::records::check_result::CheckResult;
use ulua_ast::enums::mode::Mode;
use ulua_common::FFlag;

use crate::{
  records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};
impl NonStrictTypeCheckerFixture {
  pub fn check_non_strict(&mut self, code: &str) -> CheckResult {
    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    self.get_frontend();
    let definitions = self.definitions.clone();
    let res = self.base.load_definition(&definitions, false);
    assert!(res.success);
    self
      .base
      .check_mode_string_optional_frontend_options(Mode::Nonstrict, code, None)
  }
}
