use ulua_analysis::records::frontend::Frontend;

use crate::{
  functions::register_hidden_types::register_hidden_types,
  records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
};

impl NonStrictTypeCheckerFixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    let already_initialized = self.base.frontend.is_some();

    if !already_initialized {
      // (a) 类 `as *mut Frontend` 绕道已消除：register_hidden_types 只动
      // globals/binding 表（cpp 同款登记），经现取的 `&mut Frontend` 直传，
      // 借用句尾归还后再走 register_test_types 的 fixture 借用，先后不重叠。
      register_hidden_types(self.base.get_frontend());
      self.base.register_test_types();
    }

    self.base.get_frontend()
  }
}
