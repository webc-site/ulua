use alloc::{sync::Arc, vec::Vec};

use ulua_ast::records::deprecated_info::DeprecatedInfo;

use crate::{
  records::{
    function_argument::FunctionArgument, function_definition::FunctionDefinition,
    magic_function::MagicFunction, type_level::TypeLevel,
  },
  type_aliases::{tags::Tags, type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct FunctionType {
  pub(crate) definition: Option<FunctionDefinition>,
  /// These should all be generic
  pub(crate) generics: Vec<TypeId>,
  pub(crate) generic_packs: Vec<TypePackId>,
  pub(crate) arg_names: Vec<Option<FunctionArgument>>,
  pub(crate) tags: Tags,
  pub(crate) level: TypeLevel,
  pub(crate) arg_types: TypePackId,
  pub(crate) ret_types: TypePackId,
  pub(crate) magic: Option<Arc<MagicFunction>>,

  pub(crate) has_self: bool,
  // `hasNoFreeOrGenericTypes` should be true if and only if the type does not have any free or generic types present inside it.
  // this flag is used as an optimization to exit early from procedures that manipulate free or generic types.
  pub(crate) has_no_free_or_generic_types: bool,
  pub(crate) is_checked_function: bool,
  pub(crate) is_deprecated_function: bool,
  pub(crate) deprecated_info: Option<Arc<DeprecatedInfo>>,
}
