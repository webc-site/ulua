use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnsupportedType {
  pub(crate) r#type: TypeId,
}
