use ulua_analysis::records::source_module::SourceModule;

use crate::{functions::from_string::from_string, records::fixture::Fixture};

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  pub fn get_main_source_module(&mut self) -> *mut SourceModule {
    let main_module_name = from_string(MAIN_MODULE_NAME);
    let frontend = self.get_frontend();
    frontend.get_source_module(&main_module_name) as *mut SourceModule
  }
}
