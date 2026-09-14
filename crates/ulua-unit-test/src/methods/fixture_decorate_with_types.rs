use alloc::{string::String, sync::Arc};

use ulua_analysis::{functions::attach_type_data::attach_type_data, records::module::Module};
use ulua_ast::functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block;

use crate::records::fixture::Fixture;

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  pub fn decorate_with_types(&mut self, code: &str) -> String {
    let module_name = String::from(MAIN_MODULE_NAME);
    self
      .file_resolver
      .source
      .insert(module_name.clone(), code.to_owned());

    let frontend = self.get_frontend();
    frontend.mark_dirty(&module_name, None);
    let _type_info = frontend.check_module_name_optional_frontend_options(&module_name, None);

    let source_module = frontend.get_source_module_mut(&module_name);
    assert!(!source_module.is_null());
    let module = frontend.module_resolver.get_module(&module_name);

    unsafe {
      attach_type_data(
        &mut *source_module,
        &mut *(Arc::as_ptr(&module) as *mut Module),
      );
      pretty_print_with_types_ast_stat_block(&mut *(*source_module).root)
    }
  }
}
