use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneralizationResult {
  pub result: Option<TypeId>,
  pub was_replaced_by_generic: bool,
  pub resource_limits_exceeded: bool,
}

unsafe impl Send for GeneralizationResult {}
unsafe impl Sync for GeneralizationResult {}
