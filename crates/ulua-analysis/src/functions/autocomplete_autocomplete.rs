//! Source: `Analysis/src/Autocomplete.cpp`

use ulua_ast::records::position::Position;
use ulua_common::macros::{
  luau_assert::LUAU_ASSERT,
  luau_timetrace_scope::{LUAU_TIMETRACE_ARGUMENT, LUAU_TIMETRACE_SCOPE},
};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    autocomplete_autocomplete_core::{AutocompleteArgs, autocomplete_},
    find_ancestry_at_position_for_autocomplete_ast_query::find_ancestry_at_position_for_autocomplete_source_module_position,
    find_scope_at_position::find_scope_at_position,
    is_within_comment_module::is_within_comment_source_module_position,
    is_within_hot_comment_module::is_within_hot_comment_source_module_position,
    shared_mut::shared_mut,
  },
  records::{
    arena_handle::Handle, autocomplete_result::AutocompleteResult, frontend::Frontend,
    type_arena::TypeArena,
  },
  type_aliases::{
    module_name_type::ModuleName, string_completion_callback::StringCompletionCallback,
  },
};
pub fn autocomplete(
  frontend: &mut Frontend,
  module_name: &ModuleName,
  position: Position,
  callback: StringCompletionCallback,
) -> AutocompleteResult {
  LUAU_TIMETRACE_SCOPE!("Luau::autocomplete", "Autocomplete");
  LUAU_TIMETRACE_ARGUMENT!("name", module_name.as_str());

  let Some(source_module) = frontend.get_source_module(module_name) else {
    return AutocompleteResult::new();
  };

  // C++：`module` 为可能为空的 ModulePtr，模块尚未完成类型检查时 `if (!module) return {}`。
  let module = if frontend.get_luau_solver_mode() == SolverMode::New {
    frontend.module_resolver.try_get_module(module_name)
  } else {
    frontend
      .module_resolver_for_autocomplete
      .try_get_module(module_name)
  };
  let Some(module) = module else {
    return AutocompleteResult::new();
  };

  // 经 `Frontend::builtin_types_ref` chokepoint 取内建单例共享引用（自指针
  // 布线契约集中于 chokepoint，调用点免 unsafe）。
  let builtin_types = frontend.builtin_types_ref();

  let global_scope = if frontend.get_luau_solver_mode() == SolverMode::New {
    shared_mut(&frontend.globals.global_scope)
  } else {
    shared_mut(&frontend.globals_for_autocomplete.global_scope)
  };

  let mut type_arena = TypeArena::default();
  // `source_module` 为 `Option<Handle>` 交付的表内实例别名句柄，`get` 物化
  // 共享只读借用（解引用契约集中在 `records/arena_handle.rs`），三处使用
  // 提到此处一次取得，与原逐处 `&*source_module` 同值。
  let source = source_module.get();
  let is_in_hot_comment = is_within_hot_comment_source_module_position(source, position);
  if is_within_comment_source_module_position(source, position) && !is_in_hot_comment {
    return AutocompleteResult::new();
  }

  let mut ancestry =
    find_ancestry_at_position_for_autocomplete_source_module_position(source, position);
  LUAU_ASSERT!(!ancestry.is_empty());
  // `findScopeAtPosition` returns a (nullable) ScopePtr; modeled here as
  // `Option<ScopePtr>`. `autocomplete_` takes `&ScopePtr`, so unwrap the
  // resolved scope (the empty-scopes corner is degenerate).
  // Safety: 仅当 `module.scopes` 为空才返回 None；上方 ancestry 的
  // LUAU_ASSERT 非空已确认本模块 parse 产出作用域表，scopes 非空蕴含
  // find_scope_at_position 命中 Some，None 分支不可达。
  let start_scope = find_scope_at_position(&module, position)
    .expect("scopes 非空时 find_scope_at_position 必命中 Some（见上 Safety）");

  // autocomplete_ 为安全入口：module/ancestry/type_arena/scope 等借用的存活期
  // 由上方构造与栈上持有保证，无需再包 unsafe。
  autocomplete_(AutocompleteArgs {
    module: &module,
    builtin_types,
    type_arena: Handle::from_mut(&mut type_arena),
    ancestry: &mut ancestry,
    global_scope,
    scope_at_position: &start_scope,
    position,
    file_resolver: Some(frontend.file_resolver_ref()),
    callback,
    is_in_hot_comment,
  })
}
