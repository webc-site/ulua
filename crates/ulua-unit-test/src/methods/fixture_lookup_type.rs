use alloc::string::String;

use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn lookup_type(&mut self, name: &str) -> Option<TypeId> {
    let module = self.get_main_module(false);
    if module.is_null() {
      panic!("lookupType: No main module");
    }

    let module = unsafe {
      // Safety: module 行 10-12 判空 panic 后存活：主模块由 resolver 注册持有（Arc）直至 fixture 结束；&* 物化只读借用做 lookup_type 查询，引用不逃逸。
      &*module
    };
    if !module.has_module_scope() {
      return None;
    }

    let scope = module.get_module_scope();
    // 依赖 Scope::lookup_type 接收 &Name(=String)
    let type_fun = scope.lookup_type(&String::from(name))?;
    Some(type_fun.r#type())
  }
}
