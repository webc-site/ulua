use alloc::string::String;

use ulua_analysis::{
  functions::{freeze::freeze, unfreeze::unfreeze},
  records::{frontend::Frontend, load_definition_file_result::LoadDefinitionFileResult},
};

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn load_definition(
    &mut self,
    source: &str,
    for_autocomplete: bool,
  ) -> LoadDefinitionFileResult {
    let frontend = self.get_frontend();
    let frontend_ptr = frontend as *mut Frontend;

    let result = unsafe {
      let globals = if for_autocomplete {
        &mut (*frontend_ptr).globals_for_autocomplete
      } else {
        &mut (*frontend_ptr).globals
      };

      unfreeze(globals.global_types_mut());
      let target_scope = globals.global_scope();
      let result = (*frontend_ptr).load_definition_file(
        globals,
        target_scope,
        source,
        String::from("@test"),
        false,
        for_autocomplete,
      );
      freeze(globals.global_types_mut());
      result
    };

    assert!(
      result.success,
      "loadDefinition: unable to load definition file"
    );
    result
  }
}
