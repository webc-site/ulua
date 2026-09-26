use crate::{
  records::fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl FragmentAutocompleteTypeCloner {
  pub fn shallow_clone_type_id(&mut self, ty: TypeId) -> TypeId {
    self.base.shallow_clone_type_id(ty)
  }

  pub fn shallow_clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    self.base.shallow_clone_type_pack_id(tp)
  }
}
