//! Faithful port of `TypeChecker2::findInnermostScope` (TypeChecker2.cpp:609-629).
use ulua_ast::records::location::Location;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{scope::Scope, scope_registry::resolve_scope, type_checker_2::TypeChecker2},
};

impl TypeChecker2 {
  pub fn find_innermost_scope(&self, location: Location) -> *mut Scope {
    // Scope* bestScope = module->getModuleScope().get();
    // Safety: self.module 是 TypeChecker2 构造接线的非空 `*mut Module`（NotNull），指向当前模块、
    // 本次检查期内存活；get_module_scope 取 &self 只读。
    let module_scope = unsafe { (*self.module).get_module_scope() };
    let mut best_scope: *mut Scope = arc_as_mut(&module_scope);

    let mut did_narrow;
    loop {
      did_narrow = false;
      // children 持 ScopeId 句柄（NotNull<Scope*> 的句柄化）：只读经 resolve_scope
      // 取子 scope 判 location，命中后还原为同址裸指针继续下钻（起点为函数局部
      // Arc，之后为注册表保活的子 Scope，存活前提与迁移前逐字相同）。
      // Safety: best_scope 恒指向存活 Scope，此处仅只读借用 children 遍历。
      for &child_id in unsafe { &(*best_scope).children } {
        let Some(child) = resolve_scope(child_id) else {
          continue;
        };
        if child.location.encloses(&location) {
          best_scope = child as *const Scope as *mut Scope;
          did_narrow = true;
          break;
        }
      }

      // Safety: best_scope 指向存活 Scope（同上），此处仅只读判定 children 是否为空。
      if !(did_narrow && unsafe { !(*best_scope).children.is_empty() }) {
        break;
      }
    }

    best_scope
  }
}
