use ulua_ast::records::location::Location;

use crate::{
  records::{generic_type_definition::GenericTypeDefinition, type_fun::TypeFun},
  type_aliases::type_id::TypeId,
};

impl TypeFun {
  pub fn type_fun_vector_generic_type_definition_type_id_optional_location(
    type_params: Vec<GenericTypeDefinition>,
    r#type: TypeId,
    definition_location: Option<Location>,
  ) -> Self {
    Self {
      type_params,
      type_pack_params: Vec::new(),
      r#type,
      definition_location,
    }
  }
}
