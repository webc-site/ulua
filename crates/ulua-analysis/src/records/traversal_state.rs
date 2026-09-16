use crate::{
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
  type_aliases::type_or_pack::TypeOrPack,
};
#[derive(Debug, Clone)]
pub struct TraversalState {
  pub(crate) current: TypeOrPack,
  pub(crate) builtin_types: *const BuiltinTypes,
  pub(crate) arena: *mut TypeArena,
  pub(crate) steps: i32,
  pub(crate) encountered_error_suppression: bool,
}
