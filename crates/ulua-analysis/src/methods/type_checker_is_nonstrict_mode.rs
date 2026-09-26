use ulua_ast::enums::mode::Mode;

use crate::records::type_checker::TypeChecker;

impl TypeChecker {
  pub fn is_nonstrict_mode(&self) -> bool {
    let module = self.expect_current_module();
    matches!(module.mode, Mode::Nonstrict | Mode::NoCheck)
  }
}
