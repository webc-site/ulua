use crate::records::primitive_type::{PrimitiveType, Type};

impl PrimitiveType {
  pub fn primitive_type_type_item(r#type: Type) -> Self {
    Self {
      r#type,
      metatable: None,
    }
  }
}
