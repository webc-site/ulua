use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::records::{scope::Scope, scope_registry::resolve_scope};

impl Scope {
  pub fn is_invalid_type_alias(&self, name: &str) -> Option<Location> {
    // DenseHashMap<String, _>::find 仅收 &String（ulua-common 无 &str 查询口），键转换提出循环外，每次调用至多一次堆分配。
    let key = String::from(name);
    let mut scope: Option<&Scope> = Some(self);
    while let Some(current_scope) = scope {
      if let Some(loc) = current_scope.invalid_type_aliases.find(&key) {
        return Some(*loc);
      }

      scope = current_scope.parent.and_then(resolve_scope);
    }
    None
  }
}
