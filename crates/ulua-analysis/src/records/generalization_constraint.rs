use alloc::vec::Vec;

use ulua_ast::records::deprecated_info::DeprecatedInfo;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Default)]
pub struct GeneralizationConstraint {
  pub(crate) generalized_type: TypeId,
  pub(crate) source_type: TypeId,
  pub interior_types: Vec<TypeId>,
  pub(crate) has_deprecated_attribute: bool,
  pub(crate) deprecated_info: DeprecatedInfo,
  pub(crate) no_generics: bool,
}
