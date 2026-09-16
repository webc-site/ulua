use ulua_analysis::records::builtin_type_functions::BuiltinTypeFunctions;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn get_builtin_type_functions(&mut self) -> &BuiltinTypeFunctions {
    let builtins = self.get_builtins();
    &builtins.type_functions
  }
}
