use crate::{
  records::{scope::Scope, type_fun::TypeFun},
  type_aliases::name_type::Name,
};

impl Scope {
  pub fn lookup_imported_type(&self, module_alias: &Name, name: &Name) -> Option<TypeFun> {
    let mut scope: Option<&Scope> = Some(self);
    while let Some(current_scope) = scope {
      if let Some(imported_bindings) = current_scope.imported_type_bindings.get(module_alias)
        && let Some(type_fun) = imported_bindings.get(name)
      {
        return Some(type_fun.clone());
      }
      scope = current_scope.parent.as_ref().map(|p| p.as_ref());
    }
    None
  }
}
