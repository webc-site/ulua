use ulua_ast::records::location::Location;

use crate::records::{
  non_strict_type_checker::NonStrictTypeChecker, scope::Scope, scope_registry::resolve_scope,
};

impl NonStrictTypeChecker {
  pub fn find_innermost_scope(&self, location: Location) -> *mut Scope {
    let mut best_scope: *mut Scope =
      // Safety: `self.module` 为构造期接线的非空存活 `*mut Module`；`get_module_scope(&self)`
      // 只读返回模块根作用域 Arc，其 `as_ref()` 得 `&Scope` 并转为 `*mut` 仅作后续只读遍历，
      // 不通过该指针写入。根作用域由模块保活，地址在遍历期内稳定。
      unsafe { (*self.module).get_module_scope().as_ref() as *const Scope as *mut Scope };

    let mut did_narrow;
    loop {
      did_narrow = false;
      // Safety: `best_scope` 始终指向存活作用域（初为模块根，后为 children 句柄
      // 经注册表还原的子作用域）；`Scope.children: Vec<ScopeId>` 为原
      // NotNull<Scope*> 的句柄化，元素恒指向存活 Scope（scope_registry 契约 1），
      // 只读判 `location.encloses`、`children.is_empty()`，方法为 `&self`、
      // 全程仅共享借用，无别名冲突。
      unsafe {
        for &child_id in (*best_scope).children.iter() {
          let Some(child) = resolve_scope(child_id) else {
            continue;
          };
          if child.location.encloses(&location) {
            best_scope = child as *const Scope as *mut Scope;
            did_narrow = true;
            break;
          }
        }

        if !(did_narrow && !(*best_scope).children.is_empty()) {
          break;
        }
      }
    }

    best_scope
  }
}
