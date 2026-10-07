use crate::{
  records::{generic_type_visitor::GenericTypeVisitorTrait, has_free_type::HasFreeType},
  type_aliases::type_id::TypeId,
};

pub fn has_free_type(ty: TypeId) -> bool {
  let mut hft = HasFreeType::new();
  hft.traverse_type_id(ty);
  hft.result
}
