use ulua_analysis::records::builtin_types::BuiltinTypes;

use crate::records::tf_fixture::TfFixture;

impl TfFixture {
  pub fn get_builtins(&mut self) -> &mut BuiltinTypes {
    &mut self.builtin_types
  }
}
