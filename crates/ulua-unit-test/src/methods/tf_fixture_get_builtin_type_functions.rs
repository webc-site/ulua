use ulua_analysis::records::builtin_type_functions::BuiltinTypeFunctions;

use crate::records::tf_fixture::TfFixture;

impl TfFixture {
  pub fn get_builtin_type_functions(&mut self) -> &BuiltinTypeFunctions {
    &self.builtin_types.type_functions
  }
}
