use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{
  enums::solver_mode::SolverMode,
  records::{builtin_types::BuiltinTypes, source_module::SourceModule, type_arena::TypeArena},
  type_aliases::{module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr},
};
#[derive(Debug)]
pub struct GlobalTypes {
  pub(crate) builtin_types: NonNull<BuiltinTypes>,
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
