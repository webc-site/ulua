use ulua_ast::records::location::Location;

use crate::records::{scope::Scope, scope_registry::resolve_scope};

impl Scope {
  pub fn find_narrowest_scope_containing(&mut self, location: Location) -> *mut Scope {
    let mut best_scope = self as *mut Scope;

    loop {
      let mut did_narrow = false;
      // Safety: `best_scope` 初值为 `self as *mut Scope`（有效），之后每次重赋值为
      // `children` 句柄经 `resolve_scope` 还原的注册表存活节点（scope_registry
      // 契约 1：children 的 ScopeId 恒指向仍被 Arc 保活的子作用域，地址即原
      // NotNull<Scope*> 同址）。取共享引用读 `children` 合法；本遍历单线程，
      // 仅共享借用无 &mut 别名。
      let children = unsafe { &(*best_scope).children };

      for &child_id in children {
        let Some(child) = resolve_scope(child_id) else {
          continue;
        };
        if child.location.encloses(&location) {
          best_scope = child as *const Scope as *mut Scope;
          did_narrow = true;
          break;
        }
      }

      // Safety: `best_scope` 始终指向存活作用域（初值 self，后为 children 句柄
      // 还原的子作用域），仅只读判断其 `children` 是否为空，无别名冲突。
      if !did_narrow || unsafe { (*best_scope).children.is_empty() } {
        break;
      }
    }

    best_scope
  }
}
