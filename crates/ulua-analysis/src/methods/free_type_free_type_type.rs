use crate::{
  functions::fresh_index::fresh_index,
  records::{free_type::FreeType, type_level::TypeLevel},
  type_aliases::type_id::TypeId,
};

impl FreeType {
  pub fn free_type_type_level_type_id_type_id(
    &mut self,
    level: TypeLevel,
    lower_bound: TypeId,
    upper_bound: TypeId,
  ) {
    self.index = fresh_index();
    self.level = level;
    self.lower_bound = lower_bound;
    self.upper_bound = upper_bound;
  }
}
