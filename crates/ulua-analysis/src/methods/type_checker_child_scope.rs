use alloc::sync::Arc;

use ulua_ast::records::location::Location;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{scope::Scope, scope_registry::register_scope, type_checker::TypeChecker},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeChecker {
  pub fn child_scope(&mut self, parent: &ScopePtr, location: &Location) -> ScopePtr {
    let mut scope_value = Scope::new(parent, 0);
    scope_value.level = parent.level;
    scope_value.vararg_pack = parent.vararg_pack;
    scope_value.location = *location;
    scope_value.return_type = parent.return_type;

    self.register_child_scope(parent, scope_value, location)
  }

  /// 两处子作用域构造（`child_scope`/`child_function_scope`）的公共尾段：
  /// Arc 装箱 → 注册发放句柄 → 挂父 `children` → 登记进 `module.scopes`
  /// （cpp TypeChecker.cpp 两函数同型尾段）。差异只在前置的 Scope 字段
  /// 初始化，由调用方完成后整体传入。
  pub(crate) fn register_child_scope(
    &mut self,
    parent: &ScopePtr,
    scope_value: Scope,
    location: &Location,
  ) -> ScopePtr {
    let scope = Arc::new(scope_value);
    // 注册发放本 scope 的句柄，父 children 只存句柄（裸地址仅经注册点入系统）。
    let scope_id = register_scope(&scope);

    unsafe {
      let parent_mut = arc_as_mut(parent);
      (*parent_mut).children.push(scope_id);

      let module = arc_as_mut(self.expect_current_module());
      (*module).scopes.push((*location, scope.clone()));
    }

    scope
  }
}
