//! C++ `std::pair<FragmentTypeCheckStatus, FragmentTypeCheckResult> typecheckFragment(...)`
//! (FragmentAutocomplete.cpp:1291-1335).
use alloc::{sync::Arc, vec::Vec};
use core::ptr::null;

use ulua_ast::records::{
  ast_name_table::AstNameTable, ast_stat_block::AstStatBlock, position::Position,
};
use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  enums::fragment_type_check_status::FragmentTypeCheckStatus,
  functions::{
    find_closest_scope::find_closest_scope, get_module_resolver::get_module_resolver,
    is_within_comment_module::is_within_comment, parse_fragment::parse_fragment,
    report_fragment_string::report_fragment_string,
    typecheck_fragment_fragment_autocomplete::typecheck_fragment_,
  },
  records::{
    fragment_type_check_result::FragmentTypeCheckResult, frontend::Frontend,
    frontend_options::FrontendOptions,
    i_fragment_autocomplete_reporter::IFragmentAutocompleteReporter, module::Module, scope::Scope,
  },
  type_aliases::module_name_type::ModuleName,
};
fn empty_result() -> FragmentTypeCheckResult {
  FragmentTypeCheckResult {
    incremental_module: None,
    fresh_scope: Arc::new(Scope::scope_type_pack_id(null())),
    ancestry: Vec::new(),
  }
}

pub fn typecheck_fragment(
  frontend: &mut Frontend,
  module_name: &ModuleName,
  cursor_pos: &Position,
  opts: Option<FrontendOptions>,
  src: &str,
  fragment_end_position: Option<Position>,
  recent_parse: *mut AstStatBlock,
  reporter: *mut dyn IFragmentAutocompleteReporter,
) -> (FragmentTypeCheckStatus, FragmentTypeCheckResult) {
  typecheck_fragment_impl(
    frontend,
    module_name,
    cursor_pos,
    opts,
    src,
    fragment_end_position,
    recent_parse,
    reporter,
  )
}

// 内部实现：裸指针解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
fn typecheck_fragment_impl(
  frontend: &mut Frontend,
  module_name: &ModuleName,
  cursor_pos: &Position,
  opts: Option<FrontendOptions>,
  src: &str,
  fragment_end_position: Option<Position>,
  recent_parse: *mut AstStatBlock,
  reporter: *mut dyn IFragmentAutocompleteReporter,
) -> (FragmentTypeCheckStatus, FragmentTypeCheckResult) {
  LUAU_TIMETRACE_SCOPE!("Luau::typecheckFragment", "FragmentAutocomplete");

  if !frontend.all_module_dependencies_valid(
    module_name,
    opts.as_ref().map(|o| o.for_autocomplete).unwrap_or(false),
  ) {
    return (FragmentTypeCheckStatus::SkipAutocomplete, empty_result());
  }

  let module = {
    let resolver = get_module_resolver(frontend, opts.clone());
    resolver.get_module(module_name)
  };
  // C++: if (!module) LUAU_ASSERT(!"Expected Module for fragment typecheck"); get_module
  // panics on a missing module in this port, mirroring that assertion.

  let module_ptr = Arc::as_ptr(&module) as *mut Module;
  let names: *mut AstNameTable = unsafe {
    Arc::as_ptr(
      (*module_ptr)
        .names
        .as_ref()
        .expect("module must have names"),
    ) as *mut AstNameTable
  };
  let stale_root = unsafe { (*module_ptr).root };

  let try_parse = unsafe {
    parse_fragment(
      stale_root,
      recent_parse,
      names,
      src,
      cursor_pos,
      fragment_end_position,
    )
  };

  let parse_result = match try_parse {
    Some(pr) => pr,
    None => return (FragmentTypeCheckStatus::SkipAutocomplete, empty_result()),
  };

  if is_within_comment(
    &parse_result.comment_locations,
    fragment_end_position.unwrap_or(*cursor_pos),
  ) {
    return (FragmentTypeCheckStatus::SkipAutocomplete, empty_result());
  }

  let frontend_options = opts.clone().unwrap_or_else(|| frontend.options.clone());
  let closest_scope = find_closest_scope(&module, &parse_result.scope_pos);

  let fragment_to_parse = parse_result.fragment_to_parse.clone();
  let ancestry = parse_result.ancestry.clone();

  let mut result = unsafe {
    typecheck_fragment_(
      frontend,
      parse_result.root,
      &module,
      &closest_scope,
      cursor_pos,
      parse_result.alloc,
      &frontend_options,
      reporter,
    )
  };
  result.ancestry = ancestry;
  report_fragment_string(reporter, &fragment_to_parse);

  (FragmentTypeCheckStatus::Success, result)
}
