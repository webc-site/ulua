use crate::{
  records::r#type::Type,
  type_aliases::{bound_type::BoundType, type_variant::TypeVariant},
};

impl From<BoundType> for Type {
  fn from(bound: BoundType) -> Self {
    Type::new(TypeVariant::Bound(bound.bound_to))
  }
}
