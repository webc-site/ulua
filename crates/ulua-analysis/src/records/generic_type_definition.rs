use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericTypeDefinition {
  pub(crate) ty: TypeId,
  pub(crate) default_value: Option<TypeId>,
}
