use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{
  enums::solver_mode::SolverMode,
  records::{builtin_types::BuiltinTypes, source_module::SourceModule, type_arena::TypeArena},
  type_aliases::{module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr},
};
#[derive(Debug)]
pub struct GlobalTypes {
  /// C++ `NotNull<BuiltinTypes>`：由 `GlobalTypes::new` 入参接线、并在
  /// `Frontend::wire_self_pointers` 落位后重布线为宿主 `Frontend` 自持
  /// `builtin_types_` 字段的自指针。`Option<NonNull>` 编码构造悬置窗口：
  /// `GlobalTypes::new` 入参即时写入 `Some`（引用非空、构造期恒有效），
  /// `wire_self_pointers` 落位后重指向宿主稳定地址。解引用一律经
  /// `builtin_types_ref` / `builtin_types_of` chokepoint，`None`（理论不可达，
  /// 入参为 `&mut`）在 chokepoint 内明确 panic，调用点免触碰裸字段。
  pub(crate) builtin_types: Option<NonNull<BuiltinTypes>>,
  pub(crate) global_types: TypeArena,
  pub(crate) global_names: SourceModule,
  pub(crate) global_scope: ScopePtr,
  pub(crate) global_type_function_scope: ScopePtr,
  pub mode: SolverMode,
  /// Definition modules whose checked types were persisted into the global
  /// scope (via `Frontend::load_definition_file` / `persist_checked_types`).
  /// The persisted `TypeId`s point into each module's `TypeArena`, so the
  /// modules must outlive this `GlobalTypes` — otherwise dropping a load
  /// result frees the arena out from under the type checker (a use-after-free;
  /// issue #6). Retained as an append-only list (keyed retention would collide:
  /// the builtins load `"@luau"` twice), tying the `Arc<Module>` lifetimes to
  /// the globals that reference them.
  pub(crate) retained_modules: Vec<ModulePtr>,
}
