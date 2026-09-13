use crate::{
  records::{scope::Scope, type_fun::TypeFun},
  type_aliases::name_type::Name,
};

impl Scope {
  pub fn add_builtin_type_binding(&mut self, name: &Name, ty_fun: &TypeFun) {
    self
      .exported_type_bindings
      .insert(name.clone(), ty_fun.clone());
    self.builtin_type_names.insert(name.clone());
  }
}
