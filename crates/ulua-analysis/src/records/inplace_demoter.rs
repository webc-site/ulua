use crate::records::{arena_handle::Handle, type_arena::TypeArena, type_level::TypeLevel};

#[derive(Debug, Clone)]
pub struct InplaceDemoter {
  pub(crate) new_level: TypeLevel,
  pub(crate) arena: Handle<TypeArena>,
}
