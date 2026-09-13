use crate::{
  records::generic_type_definition::GenericTypeDefinition, type_aliases::type_id::TypeId,
};

impl GenericTypeDefinition {
  pub fn ty(&self) -> TypeId {
    self.ty
  }
}
