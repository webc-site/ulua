use ulua_ast::records::location::Location;

use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes,
  internal_error_reporter::InternalErrorReporter, normalizer::Normalizer, scope::Scope,
  subtyping::Subtyping, type_arena::TypeArena, type_check_limits::TypeCheckLimits,
  type_function_runtime::TypeFunctionRuntime,
};
/// 对应 C++ `OverloadResolver`（OverloadResolver.h:113-134）：除 `limits` 外
/// 各成员均为非拥有 `NotNull`，此处沿用本 port 的裸指针字段约定；`limits`
/// 对应 C++ `NotNull<TypeCheckLimits> limits`（OverloadResolver.h:132），移植为
/// 非拥有共享借用 `&'a TypeCheckLimits`——**绝不拥有、绝不 drop**，与
/// `TypeFunctionContext::limits`（`NonNull`）同源共享同一个 limits 对象。
#[derive(Debug, Clone)]
pub struct OverloadResolver<'a> {
  // 句柄化：原 C++ NotNull 裸指针字段。
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) normalizer: Handle<Normalizer>,
  pub(crate) type_function_runtime: Handle<TypeFunctionRuntime>,
  pub(crate) scope: *mut Scope,
  pub(crate) ice: Handle<InternalErrorReporter>,
  pub(crate) limits: &'a TypeCheckLimits,
  pub(crate) subtyping: Subtyping,
  pub(crate) call_loc: Location,
}
