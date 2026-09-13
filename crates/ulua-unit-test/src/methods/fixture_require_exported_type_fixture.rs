use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::fixture::Fixture;

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  pub fn require_exported_type_string(&mut self, name: &str) -> TypeId {
    self.require_exported_type_module_name_string(MAIN_MODULE_NAME, name)
  }
}
