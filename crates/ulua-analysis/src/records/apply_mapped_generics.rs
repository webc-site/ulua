/// C++ `ApplyMappedGenerics` (`Subtyping.cpp`): a `Substitution` subclass, so it
/// embeds `base: Substitution` and inherits `substitute` (whose traversal
/// virtual-dispatches into the `isDirty` / `clean` / `ignoreChildren` overrides
/// installed via [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable)).
use crate::records::builtin_types::BuiltinTypes;
use crate::records::{
  arena_handle::Handle, internal_error_reporter::InternalErrorReporter, substitution::Substitution,
  subtyping_environment::SubtypingEnvironment, type_arena::TypeArena,
};
#[derive(Debug, Clone)]
pub struct ApplyMappedGenerics {
  pub(crate) base: Substitution,
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) ice_reporter: Handle<InternalErrorReporter>,
  /// 宿主环境句柄：对应 cpp `NotNull<SubtypingEnvironment>` 构造参数。
  /// 借用契约：遍历期内宿主调用方不得再使用其 `&mut SubtypingEnvironment`
  /// 借用（原裸指针形态的隐含前提，现由 [`Handle`] 模块级契约显式承载，
  /// `get`/`get_mut` 解引用收口在 arena_handle 单点）。
  pub(crate) env: Handle<SubtypingEnvironment>,
}
