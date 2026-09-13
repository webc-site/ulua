//! @interface-stub

use ulua_analysis::records::load_definition_file_result::LoadDefinitionFileResult;
use ulua_common::FFlag;

use crate::records::ac_fixture_impl::AcFixtureImpl;
impl AcFixtureImpl {
  pub fn load_definition(&mut self, source: &str) -> LoadDefinitionFileResult {
    self.get_frontend();

    let result = self.base.load_definition(source, true);

    if !FFlag::DebugLuauForceOldSolver.get() {
      self.base.load_definition(source, false);
    }

    assert!(
      result.success,
      "loadDefinition: unable to load definition file"
    );
    result
  }
}
