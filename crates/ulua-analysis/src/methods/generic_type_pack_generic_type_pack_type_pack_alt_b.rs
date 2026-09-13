use crate::{
  functions::fresh_index::fresh_index,
  records::{generic_type_pack::GenericTypePack, type_level::TypeLevel},
  type_aliases::name_type::Name,
};

impl GenericTypePack {
  pub fn generic_type_pack_type_level(&mut self, level: TypeLevel) {
    self.index = fresh_index();
    self.level = level;
    self.name = Name::from(format!("g{}", self.index).as_str());
  }
}
