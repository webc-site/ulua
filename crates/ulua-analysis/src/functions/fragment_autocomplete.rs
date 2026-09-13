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
    autocomplete_autocomplete_core::{AutocompleteArgs, autocomplete_},
    freeze::freeze,
    report_waypoint::report_waypoint,
    typecheck_fragment_fragment_autocomplete_alt_b::typecheck_fragment,
    unfreeze::unfreeze,
  },
  records::{
    autocomplete_result::AutocompleteResult,
    fragment_autocomplete_result::FragmentAutocompleteResult, frontend::Frontend,
    frontend_options::FrontendOptions,
    i_fragment_autocomplete_reporter::IFragmentAutocompleteReporter, module::Module, scope::Scope,
  },
  type_aliases::{
    module_name_type::ModuleName, string_completion_callback::StringCompletionCallback,
  },
};
/// `fragment_autocomplete` 的参数包（对应 C++ 十参数签名 FragmentAutocomplete.h:121-132）。
pub struct FragmentAutocompleteArgs<'a> {
  pub frontend: &'a mut Frontend,
  pub src: &'a str,
  pub module_name: &'a ModuleName,
  pub cursor_position: Position,
  pub opts: Option<FrontendOptions>,
  pub callback: StringCompletionCallback,
  pub fragment_end_position: Option<Position>,
  pub recent_parse: *mut AstStatBlock,
  pub reporter: *mut dyn IFragmentAutocompleteReporter,
  pub is_in_hot_comment: bool,
}

pub fn fragment_autocomplete(args: FragmentAutocompleteArgs<'_>) -> FragmentAutocompleteResult {
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

  report_waypoint(reporter, FragmentAutocompleteWaypoint::TypecheckFragmentEnd);

  let global_scope = if for_autocomplete {
    Arc::as_ptr(&frontend.globals_for_autocomplete.global_scope) as *mut Scope
  } else {
    Arc::as_ptr(&frontend.globals.global_scope) as *mut Scope
  };

  let incremental_module = tc_result
    .incremental_module
    .take()
    .expect("typecheckFragment must have produced an incremental module on Success");
  let module_ptr = Arc::as_ptr(&incremental_module) as *mut Module;
  let builtin_types = unsafe { &*frontend.builtin_types };
  let fresh_scope = tc_result.fresh_scope.clone();

  unsafe {
    unfreeze(&mut (*module_ptr).internal_types);
  }

  let result = unsafe {
    autocomplete_(AutocompleteArgs {
      module: &incremental_module,
      builtin_types,
      type_arena: &mut (*module_ptr).internal_types,
      ancestry: &mut tc_result.ancestry,
      global_scope,
      scope_at_position: &fresh_scope,
      position: cursor_position,
      file_resolver: frontend.file_resolver,
      callback,
      is_in_hot_comment,
    })
  };

  unsafe {
    freeze(&mut (*module_ptr).internal_types);
  }

  report_waypoint(reporter, FragmentAutocompleteWaypoint::AutocompleteEnd);

  FragmentAutocompleteResult {
    fresh_scope: Arc::as_ptr(&fresh_scope) as *mut Scope,
    incremental_module,
    ac_results: result,
  }
}
