use ulua_ast::records::location::Location;

use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes,
  internal_error_reporter::InternalErrorReporter, normalizer::Normalizer, scope::Scope,
  subtyping::Subtyping, type_arena::TypeArena, type_check_limits::TypeCheckLimits,
  type_function_runtime::TypeFunctionRuntime,
};
/// 对应 C++ `OverloadResolver`（OverloadResolver.h:113-134）：除 `limits`/`scope`
/// 外各成员为非拥有 `NotNull`（移植为 [`Handle`] 句柄）；`limits`
/// 对应 C++ `NotNull<TypeCheckLimits> limits`（OverloadResolver.h:132），
/// `scope` 对应 C++ `NotNull<Scope> scope`（OverloadResolver.h:129），二者均为
/// 非拥有共享借用 `&'a _`——**绝不拥有、绝不 drop**，与原指针直传同构。
#[derive(Debug, Clone)]
pub struct OverloadResolver<'a> {
  // 句柄化：原 C++ NotNull 裸指针字段。
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) normalizer: Handle<Normalizer>,
  pub(crate) type_function_runtime: Handle<TypeFunctionRuntime>,
  pub(crate) scope: &'a Scope,
  pub(crate) ice: Handle<InternalErrorReporter>,
  pub(crate) limits: &'a TypeCheckLimits,
  pub(crate) subtyping: Subtyping,
  pub(crate) call_loc: Location,
}
