use alloc::string::String;

use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn lookup_imported_type(&mut self, module_alias: &str, name: &str) -> Option<TypeId> {
    let module = self.get_main_module(false);
    let module = unsafe {
      // Safety: get_main_module 在 cpp 原实现同款免检解引用：fixture 约定本调用发生在 check_module 之后（主模块已注册并由 resolver 持有至 frontend 结束），故指针存活非空；&* 物化只读借用查导入类型，与 cpp 行为一致。
      &*module
    };

    if !module.has_module_scope() {
      panic!("lookupImportedType: module scope data is not available");
    }

    let scope = module.get_module_scope();
    // 依赖 Scope::lookup_imported_type 接收 &Name(=String)
    if let Some(type_fun) =
      scope.lookup_imported_type(&String::from(module_alias), &String::from(name))
    {
      return Some(type_fun.r#type());
    }

    None
  }
}
