use ulua_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn register_test_types(&mut self) {
    // (a) 类缓存裸句柄绕道已消除：改走 `Frontend::builtin_types_ref`
    // chokepoint 取 any_type 标量拷贝（借用不绑定 frontend），随后对
    // `globals` 字段的三次顺序登记（cpp addGlobalBinding 同款）全程安全借用。
    let frontend = self.get_frontend();
    let any_type = frontend.builtin_types_ref().any_type;
    for name in ["game", "workspace", "script"] {
      add_global_binding_builtin_definitions(&mut frontend.globals, name, any_type, "@luau");
    }
  }
}
