use ulua_ast::records::location::Location;

use crate::records::{scope::Scope, scope_registry::resolve_scope};

impl Scope {
  pub fn is_invalid_type_alias(&self, name: &str) -> Option<Location> {
    // r7-rc-5（承 r7-rc-4）：DenseHashMap<String, _> 已有 &str 借用查询口，
    // an2 票的循环外键物化（`let key = String::from(name)`）整体删除，零分配查询。
    let mut scope: Option<&Scope> = Some(self);
    while let Some(current_scope) = scope {
      if let Some(loc) = current_scope.invalid_type_aliases.find_str(name) {
        return Some(*loc);
      }

      scope = current_scope.parent.and_then(resolve_scope);
    }
    None
  }
}
