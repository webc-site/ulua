use alloc::string::String;

use ulua_analysis::{
  functions::find_type_at_position::find_type_at_position, type_aliases::type_id::TypeId,
};
use ulua_ast::records::position::Position;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn find_type_at_position_module_name_position(
    &mut self,
    module_name: &str,
    position: Position,
  ) -> Option<TypeId> {
    if module_name.is_empty() {
      return self.find_type_at_position_position(position);
    }

    let module_name = String::from(module_name);
    let frontend = self.get_frontend();
    let module = frontend.module_resolver.get_module(&module_name);
    let source_module = frontend.get_source_module(&module_name);

    if source_module.is_null() {
      panic!("findTypeAtPosition: No source module \"{}\"", module_name);
    }

    unsafe { find_type_at_position(&module, &*source_module, position) }
  }
}
