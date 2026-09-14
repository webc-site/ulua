use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::{
  records::property_type::Property,
  type_aliases::{tags::Tags, type_id::TypeId},
};
impl Property {
  pub fn property_type_id_bool_string_optional_location_tags_optional_string_optional_location(
    read_ty: TypeId,
    deprecated: bool,
    deprecated_suggestion: String,
    location: Option<Location>,
    tags: Tags,
    documentation_symbol: Option<String>,
    type_location: Option<Location>,
  ) -> Self {
    Self {
      deprecated,
      deprecated_suggestion,
      location,
      type_location,
      tags,
      documentation_symbol,
      read_ty: Some(read_ty),
      write_ty: Some(read_ty),
    }
  }
}
