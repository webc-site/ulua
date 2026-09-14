use alloc::{string::String, sync::Arc};

use ulua_analysis::records::module::Module;
use ulua_common::FFlag;

use crate::records::fixture::Fixture;
const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  pub fn get_main_module(&mut self, for_autocomplete: bool) -> *mut Module {
    let module_name = String::from(MAIN_MODULE_NAME);
    let frontend = self.get_frontend();
    // C++ `Fixture::getMainModule` (Fixture.cpp:430-436): the dedicated
    // autocomplete module resolver is only consulted in the old solver
    // (`forAutocomplete && FFlag::DebugLuauForceOldSolver`); otherwise the
    // regular module resolver is used even for autocomplete queries.
    let module = if for_autocomplete && FFlag::DebugLuauForceOldSolver.get() {
      frontend
        .module_resolver_for_autocomplete
        .get_module(&module_name)
    } else {
      frontend.module_resolver.get_module(&module_name)
    };

    Arc::as_ptr(&module) as *mut Module
  }
}
