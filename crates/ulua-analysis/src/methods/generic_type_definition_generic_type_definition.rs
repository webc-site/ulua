use crate::{
  records::generic_type_definition::GenericTypeDefinition, type_aliases::type_id::TypeId,
};

impl GenericTypeDefinition {
  pub fn new(ty: TypeId) -> Self {
    Self {
      ty,
      default_value: None,
    }
  }
}
