use crate::records::{builtin_types::BuiltinTypes, type_arena::TypeArena, type_ids::TypeIds};

#[derive(Debug, Clone)]
pub struct UnionBuilder {
  pub(crate) arena: *mut TypeArena,
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub(crate) options: TypeIds,
  pub(crate) is_top: bool,
}
