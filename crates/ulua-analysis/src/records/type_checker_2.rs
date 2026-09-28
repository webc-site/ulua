//! Source: `Analysis/include/Luau/TypeChecker2.h` (hand-ported; fields only)

use alloc::{string::String, vec::Vec};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::type_context::TypeContext,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, dcr_logger::DcrLogger,
    internal_error_reporter::InternalErrorReporter, module::Module, normalizer::Normalizer,
    scope::Scope, source_module::SourceModule, subtyping::Subtyping,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug)]
pub struct TypeChecker2 {
  // 句柄化：原 C++ `NotNull<BuiltinTypes/TypeFunctionRuntime/ICE>` 裸指针。
  pub builtin_types: Handle<BuiltinTypes>,
  pub type_function_runtime: Handle<TypeFunctionRuntime>,
  // 句柄化（#24 b13-tc2-fields 字段面）：原 `DcrLogger*`（可空，`Option` 承载
  // null 语义一一对应）、`NotNull<TypeCheckLimits>`、`const SourceModule*` 裸字段。
  pub logger: Option<Handle<DcrLogger>>,
  pub limits: Handle<TypeCheckLimits>,
  pub ice: Handle<InternalErrorReporter>,
  pub source_module: Handle<SourceModule>,
  // `module` 为 Arc 写穿裸句柄，独立挂账登记于 [`TypeChecker2::new_boxed`] 文档，
  // 本波不处置。
  pub module: *mut Module,
  pub type_context: TypeContext,

  // 句柄化：scope 栈元素原为 `*mut Scope`（C++ `std::vector<NotNull<Scope*>>`
  // 直译），现以 `Handle<Scope>` 编码非空，判空/存活契约集中于 `arena_handle`。
  pub stack: Vec<Handle<Scope>>,
  pub function_decl_stack: Vec<TypeId>,

  pub seen_type_function_instances: DenseHashSet<TypeId>,

  pub normalizer: Normalizer,
  pub _subtyping: Subtyping,
  // 自引用孪生对句柄化：原 `*mut Subtyping` 在 `new` 后、`wire_self_pointers`
  // 回填前以 null 悬置；现以 `Option` 承载同一窗口，回填后恒 `Some`。
  pub subtyping: Option<Handle<Subtyping>>,

  pub warned_globals: DenseHashSet<String>,
}
