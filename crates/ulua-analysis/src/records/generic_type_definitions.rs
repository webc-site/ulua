use alloc::vec::Vec;

use crate::records::{
  generic_type_definition::GenericTypeDefinition,
  generic_type_pack_definition::GenericTypePackDefinition,
};
#[derive(Debug, Clone, Default)]
pub struct GenericTypeDefinitions {
  pub generic_types: Vec<GenericTypeDefinition>,
  pub generic_packs: Vec<GenericTypePackDefinition>,
}
