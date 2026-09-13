use crate::{
  records::{scope::Scope, type_fun::TypeFun},
  type_aliases::name_type::Name,
};

impl Scope {
  pub fn lookup_type(&self, name: &Name) -> Option<TypeFun> {
    let mut current_scope = self;
    loop {
      if let Some(type_fun) = current_scope.exported_type_bindings.get(name) {
        return Some(type_fun.clone());
      }

      if let Some(type_fun) = current_scope.private_type_bindings.get(name) {
        return Some(type_fun.clone());
      }

      if let Some(parent_ptr) = &current_scope.parent {
        current_scope = parent_ptr;
      } else {
        return None;
      }
    }
  }
}
