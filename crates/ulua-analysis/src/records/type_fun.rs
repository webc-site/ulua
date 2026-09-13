use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  records::{
    generic_type_definition::GenericTypeDefinition,
    generic_type_pack_definition::GenericTypePackDefinition,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeFun {
  pub(crate) type_params: Vec<GenericTypeDefinition>,
  pub(crate) type_pack_params: Vec<GenericTypePackDefinition>,
  /// The underlying type.
  ///
  /// WARNING! This is not safe to use as a type if type_params is not empty!!
  /// You must first use TypeChecker::instantiateTypeFun to turn it into a real type.
  pub(crate) r#type: TypeId,
  /// The location of where this TypeFun was defined, if available
  pub(crate) definition_location: Option<Location>,
}

impl TypeFun {
  pub fn type_params(&self) -> &[GenericTypeDefinition] {
    &self.type_params
  }

  pub fn type_pack_params(&self) -> &[GenericTypePackDefinition] {
    &self.type_pack_params
  }

  pub fn r#type(&self) -> TypeId {
    self.r#type
  }

  pub fn definition_location(&self) -> Option<Location> {
    self.definition_location
  }
}
