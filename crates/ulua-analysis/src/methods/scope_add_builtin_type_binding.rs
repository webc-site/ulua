use crate::records::{scope::Scope, type_fun::TypeFun};

impl Scope {
  pub fn add_builtin_type_binding(&mut self, name: &str, ty_fun: &TypeFun) {
    self
      .exported_type_bindings
      .insert(name.to_string(), ty_fun.clone());
    self.builtin_type_names.insert(name.to_string());
  }
}
