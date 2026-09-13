use crate::{
  functions::fresh_index::fresh_index, records::generic_type_pack::GenericTypePack,
  type_aliases::name_type::Name,
};

impl GenericTypePack {
  pub fn generic_type_pack(&mut self) {
    self.index = fresh_index();
    self.name = Name::from(format!("g{}", self.index).as_str());
  }
}
