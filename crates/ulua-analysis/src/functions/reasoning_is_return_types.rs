use crate::{
  enums::pack_field::PackField, records::path::Path, type_aliases::component::Component,
};
pub fn reasoning_is_return_types(path: &Path) -> bool {
  if path.components.is_empty() {
    return false;
  }

  let first_component = &path.components[0];

  if let Component::PackField(field) = first_component {
    *field == PackField::Returns
  } else {
    false
  }
}
