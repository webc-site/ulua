use ulua_ast::enums::mode::Mode;

use crate::records::type_checker::TypeChecker;

impl TypeChecker {
  pub fn is_nonstrict_mode(&self) -> bool {
    let module = self.current_module.as_ref().unwrap();
    module.mode == Mode::Nonstrict || module.mode == Mode::NoCheck
  }
}
