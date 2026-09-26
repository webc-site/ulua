use ulua_ast::enums::mode::Mode;

use crate::records::type_checker::TypeChecker;

impl TypeChecker {
  pub fn is_nonstrict_mode(&self) -> bool {
    let module = self.current_module.as_ref().expect("current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some");
    matches!(module.mode, Mode::Nonstrict | Mode::NoCheck)
  }
}
