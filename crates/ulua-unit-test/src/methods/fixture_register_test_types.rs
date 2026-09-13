use ulua_analysis::{
  functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions,
  records::frontend::Frontend,
};

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn register_test_types(&mut self) {
    let any_type = unsafe { (*self.builtin_types).any_type };
    let frontend = self.get_frontend();
    let frontend_ptr = frontend as *mut Frontend;

    unsafe {
      add_global_binding_builtin_definitions(
        &mut (*frontend_ptr).globals,
        "game",
        any_type,
        "@luau",
      );
      add_global_binding_builtin_definitions(
        &mut (*frontend_ptr).globals,
        "workspace",
        any_type,
        "@luau",
      );
      add_global_binding_builtin_definitions(
        &mut (*frontend_ptr).globals,
        "script",
        any_type,
        "@luau",
      );
    }
  }
}
