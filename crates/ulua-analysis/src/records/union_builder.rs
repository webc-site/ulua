use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes, type_arena::TypeArena, type_ids::TypeIds,
};

#[derive(Debug, Clone)]
pub struct UnionBuilder {
  // 原 `arena`/`builtin_types` 为照抄 C++ 引用/`NotNull` 形参的裸指针；
  // 目标均为构造期由宿主接线的存活会话级实例、恒非空，收敛为 Handle。
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) options: TypeIds,
  pub(crate) is_top: bool,
}
