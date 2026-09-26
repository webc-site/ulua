//! Source: `Analysis/src/Autocomplete.cpp`

use ulua_ast::records::position::Position;
use ulua_common::macros::{
  luau_assert::LUAU_ASSERT,
  luau_timetrace_scope::{LUAU_TIMETRACE_ARGUMENT, LUAU_TIMETRACE_SCOPE},
};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    arc_as_mut::arc_as_mut,
    autocomplete_autocomplete_core::{AutocompleteArgs, autocomplete_},
    find_ancestry_at_position_for_autocomplete_ast_query::find_ancestry_at_position_for_autocomplete_source_module_position,
    find_scope_at_position::find_scope_at_position,
    is_within_comment_module::is_within_comment_source_module_position,
    is_within_hot_comment_module::is_within_hot_comment_source_module_position,
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
    arc_as_mut(&frontend.globals.global_scope)
  } else {
    arc_as_mut(&frontend.globals_for_autocomplete.global_scope)
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

  // Safety: 满足 autocomplete_ 的 # Safety 契约——module 为上方 try_get_module 命中
  // 的 Arc 克隆且存活；builtin_types 由构造布线的 NonNull 取得（NotNull 语义）；
  // ancestry 元素源自存活模块 AST 的 arena 节点；type_arena 为栈上局部值；
  // global_scope 与 scope_at_position 均为存活 Arc<Scope> 的写句柄/共享引用；
  // file_resolver 经 `file_resolver_ref` 共享读取后以 `Some` 传入（构造布线恒非空，
  // 下游 null 折叠分支不可达）。
  unsafe {
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
}
