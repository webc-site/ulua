//! C++ `FragmentAutocompleteResult fragmentAutocomplete(...)`
//! (FragmentAutocomplete.cpp:1373-1413).
use alloc::sync::Arc;
use core::ptr::null_mut;

use ulua_ast::records::{ast_stat_block::AstStatBlock, position::Position};
use ulua_common::macros::{
  luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE,
};

use crate::{
  enums::{
    fragment_autocomplete_waypoint::FragmentAutocompleteWaypoint,
    fragment_type_check_status::FragmentTypeCheckStatus,
  },
  functions::{
    arc_as_mut::arc_as_mut,
    autocomplete_autocomplete_core::{AutocompleteArgs, autocomplete_},
    freeze::freeze,
    typecheck_fragment_fragment_autocomplete::typecheck_fragment,
    unfreeze::unfreeze,
  },
  records::{
    arena_handle::Handle, autocomplete_result::AutocompleteResult,
    fragment_autocomplete_result::FragmentAutocompleteResult, frontend::Frontend,
    frontend_options::FrontendOptions, i_fragment_autocomplete_reporter::ReporterRef,
    module::Module,
  },
  type_aliases::{
    module_name_type::ModuleName, string_completion_callback::StringCompletionCallback,
  },
};
/// `fragment_autocomplete` 的参数包（对应 C++ 十参数签名 FragmentAutocomplete.h:121-132）。
pub struct FragmentAutocompleteArgs<'a, 'b> {
  pub frontend: &'a mut Frontend,
  pub src: &'a str,
  pub module_name: &'a ModuleName,
  pub cursor_position: Position,
  pub opts: Option<FrontendOptions>,
  pub callback: StringCompletionCallback,
  pub fragment_end_position: Option<Position>,
  pub recent_parse: *mut AstStatBlock,
  pub reporter: ReporterRef<'b>,
  pub is_in_hot_comment: bool,
}

pub fn fragment_autocomplete(args: FragmentAutocompleteArgs<'_, '_>) -> FragmentAutocompleteResult {
  // 按原参数顺序解包，保持与 C++ 一一对应。
  let FragmentAutocompleteArgs {
    frontend,
    src,
    module_name,
    cursor_position,
    opts,
    callback,
    fragment_end_position,
    recent_parse,
    reporter,
    is_in_hot_comment,
  } = args;
  LUAU_TIMETRACE_SCOPE!("Luau::fragmentAutocomplete", "FragmentAutocomplete");
  LUAU_TIMETRACE_ARGUMENT!("name", module_name.as_str());

  let for_autocomplete = opts.as_ref().map(|o| o.for_autocomplete).unwrap_or(false);

  let (tc_status, mut tc_result) = typecheck_fragment(
    frontend,
    module_name,
    &cursor_position,
    opts,
    src,
    fragment_end_position,
    recent_parse,
    reporter,
  );

  if tc_status == FragmentTypeCheckStatus::SkipAutocomplete {
    // C++ `return {}` — a default FragmentAutocompleteResult.
    return FragmentAutocompleteResult {
      incremental_module: Arc::new(Module::default()),
      fresh_scope: null_mut(),
      ac_results: AutocompleteResult::new(),
    };
  }

  reporter.report_waypoint(FragmentAutocompleteWaypoint::TypecheckFragmentEnd);

  let global_scope = if for_autocomplete {
    arc_as_mut(&frontend.globals_for_autocomplete.global_scope)
  } else {
    arc_as_mut(&frontend.globals.global_scope)
  };

  let incremental_module = tc_result
    .incremental_module
    .take()
    .expect("typecheckFragment must have produced an incremental module on Success");
  let module_ptr = arc_as_mut(&incremental_module);
  // 经 `Frontend::builtin_types_ref` chokepoint 取内建单例共享引用（自指针布线
  // 契约集中于 chokepoint，调用点免 unsafe）。
  let builtin_types = frontend.builtin_types_ref();
  let fresh_scope = tc_result.fresh_scope.clone();

  // Safety: module_ptr 是 arc_as_mut 从本函数栈上持有的 incremental_module
  // （Arc<Module>）取得的写句柄——非空、对齐、Arc 存活至函数尾；internal_types 是
  // 该 module 自有的会话 TypeArena，单线程串行独占（arc_as_mut 惯用法），故在函数
  // 头一次取得 &mut，供 unfreeze/autocomplete_/freeze 依次短借用，全程无第二处
  // arena 可变句柄。
  let module_arena = unsafe { &mut (*module_ptr).internal_types };

  unfreeze(&mut *module_arena);

  let result = unsafe {
    // Safety: 满足 autocomplete_ 的 # Safety 契约——module 为上方存活的
    // Arc<Module> 借用；builtin_types 由构造布线 NonNull 取得（NotNull 语义）；
    // type_arena 是 module 自有 arena 的独占写句柄（上一行已 unfreeze，紧随
    // 语句后 freeze 归还）；ancestry/scope_at_position 借自本函数栈上持有值；
    // global_scope 为 frontend 全局 Arc<Scope> 的 arc_as_mut 写句柄（单线程
    // 独占写惯用法）；file_resolver 是 Frontend 构造布线的 NonNull 指针，恒非空。
    autocomplete_(AutocompleteArgs {
      module: &incremental_module,
      builtin_types,
      type_arena: Handle::from_mut(&mut *module_arena),
      ancestry: &mut tc_result.ancestry,
      global_scope,
      scope_at_position: &fresh_scope,
      position: cursor_position,
      file_resolver: Some(frontend.file_resolver_ref()),
      callback,
      is_in_hot_comment,
    })
  };

  freeze(&mut *module_arena);

  reporter.report_waypoint(FragmentAutocompleteWaypoint::AutocompleteEnd);

  FragmentAutocompleteResult {
    fresh_scope: arc_as_mut(&fresh_scope),
    incremental_module,
    ac_results: result,
  }
}
