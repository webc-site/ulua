//! Ported from `tests/Frontend.test.cpp`.
use ulua_analysis::{
  functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions,
  records::frontend::Frontend,
};

use crate::records::frontend_fixture::FrontendFixture;

impl FrontendFixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    let already_initialized = self.base.base.frontend.is_some();
    let frontend = self.base.get_frontend();

    if !already_initialized {
      // (a) 类裸指针绕道已消除：`base.get_frontend()` 本就是 `&mut Frontend`，
      // 直连字段借用即可（`builtin_types_ref` chokepoint 的借用不绑定
      // frontend，与随后 `&mut frontend.globals` 不重叠），与 cpp
      // addGlobalBinding 同款顺序独占登记，全程安全借用。
      let any_type = frontend.builtin_types_ref().any_type;
      for name in ["game", "script"] {
        add_global_binding_builtin_definitions(&mut frontend.globals, name, any_type, "@test");
      }
    }

    frontend
  }
}
