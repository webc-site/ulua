use ulua_analysis::{
  functions::find_expected_type_at_position::find_expected_type_at_position,
  records::{module::Module, source_module::SourceModule},
  type_aliases::type_id::TypeId,
};
use ulua_ast::records::position::Position;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn find_expected_type_at_position(&mut self, position: Position) -> Option<TypeId> {
    let module: *mut Module = self.get_main_module(false);
    let source_module: *mut SourceModule = self.get_main_source_module();
    unsafe { find_expected_type_at_position(&*module, &*source_module, position) }
  }
}
