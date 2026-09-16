use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type_pack::GenericTypePack, scope::Scope},
  type_aliases::name_type::Name,
};

impl GenericTypePack {
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
