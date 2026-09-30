use core::ptr::null_mut;

use ulua_analysis::{
  functions::follow_type_utils::follow_optional_ty,
  records::{module::Module, scope::Scope},
  type_aliases::{module_name_type::ModuleName, type_id::TypeId},
};
use ulua_common::LUAU_ASSERT;

use crate::{functions::lookup_name::lookup_name, records::fixture::Fixture};

impl Fixture {
  pub fn require_type_string(&mut self, name: &str) -> TypeId {
    let ty = self.get_type(name, false);
    LUAU_ASSERT!(ty.is_some());
    follow_optional_ty(ty).unwrap_or(null_mut())
  }
}

impl Fixture {
  pub fn require_type_module_name_string(&mut self, module_name: &str, name: &str) -> TypeId {
    let module = self
      .get_frontend()
      .module_resolver
      .get_module(&ModuleName::from(module_name));
    self.require_type_module_ptr_string(&module, name)
  }
}

impl Fixture {
  pub fn require_type_module_ptr_string(&mut self, module: &Module, name: &str) -> TypeId {
    let scope = module.get_module_scope();
    self.require_type_scope_string(&scope, name)
  }
}

impl Fixture {
  /// cpp `Fixture::requireType(Scope*, const string&)`：取不到类型即判定夹具失败。
  ///
  /// 形参收 `&Scope`（cpp 的 `Scope*` 只读使用）：`scope` 须在整个查询期内存活，
  /// 由调用方持有的 `Arc<Scope>` 克隆或模块 resolver 的强引用保证；绑定链遍历只读，
  /// 不写 scope，也不触发 GC。
  pub fn require_type_scope_string(&mut self, scope: &Scope, name: &str) -> TypeId {
    match lookup_name(scope, name) {
      Some(ty) => ty,
      // cpp requireType 在 lookupName 落空时报 InternalCompilerError；夹具里等价于用例失败。
      None => panic!("requireType: No type \"{}\"", name),
    }
  }
}
