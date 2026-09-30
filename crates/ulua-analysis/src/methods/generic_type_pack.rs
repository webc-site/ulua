//! `generic_type_pack` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type_pack::GenericTypePack, scope::Scope, type_level::TypeLevel},
  type_aliases::name_type::Name,
};

impl GenericTypePack {
  pub fn generic_type_pack(&mut self) {
    self.index = fresh_index();
    self.name = Name::from(format!("g{}", self.index).as_str());
  }

  pub fn generic_type_pack_type_level(&mut self, level: TypeLevel) {
    self.index = fresh_index();
    self.level = level;
    self.name = Name::from(format!("g{}", self.index).as_str());
  }

  pub fn generic_type_pack_type_level_name(&mut self, level: TypeLevel, name: &Name) {
    self.index = fresh_index();
    self.level = level;
    self.name = name.clone();
    self.explicit_name = true;
  }

  pub fn generic_type_pack_scope_name_polarity(
    &mut self,
    scope: *mut Scope,
    name: Name,
    polarity: Polarity,
  ) {
    self.index = fresh_index();
    self.scope = scope;
    self.name = name;
    self.explicit_name = true;
    self.polarity = polarity;
  }
}

impl GenericTypePack {
  pub fn new() -> Self {
    Self {
      index: fresh_index(),
      level: TypeLevel::default(),
      scope: null_mut(),
      name: Name::default(),
      explicit_name: false,
      polarity: Polarity::Unknown,
    }
  }

  pub fn new_name(name: Name) -> Self {
    Self {
      index: fresh_index(),
      level: TypeLevel::default(),
      scope: null_mut(),
      name,
      explicit_name: true,
      polarity: Polarity::Unknown,
    }
  }
}
impl Default for GenericTypePack {
  fn default() -> Self {
    Self::new()
  }
}
