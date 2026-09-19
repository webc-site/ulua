use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_local::AstLocal, ast_name::AstName,
    ast_stat_block::AstStatBlock, ast_stat_class::AstStatClass, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    ast_stat_type_function::AstStatTypeFunction, location::Location, position::Position,
  },
  rtti::{ast_node_as, ast_node_try_as},
};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  functions::{
    find_ancestry_at_position_for_autocomplete_ast_query_alt_b::find_ancestry_at_position_for_autocomplete_ast_stat_block_position,
    get_fragment_region_with_block_diff::get_fragment_region_with_block_diff,
  },
  records::fragment_autocomplete_ancestry_result::FragmentAutocompleteAncestryResult,
};

/// 对照 C++ `localStack.push_back(v); localMap[v->name] = v;`：
/// 登记局部变量 → 压栈 + 名字映射；null 跳过（解析器保证非空，防御兜底）。
fn add_local(
  local_stack: &mut Vec<*mut AstLocal>,
  local_map: &mut DenseHashMap<AstName, *mut AstLocal>,
  local: *mut AstLocal,
) {
  // SAFETY: local 判空后才解引用（C++ 同款前提），arena 持有。
  let Some(local_ref) = (unsafe { local.as_ref() }) else {
    return;
  };
  local_stack.push(local);
  *local_map.get_or_insert(local_ref.name) = local;
}

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_ancestry_for_fragment_parse(
  stale: *mut AstStatBlock,
  cursor_pos: Position,
  last_good_parse: *mut AstStatBlock,
) -> FragmentAutocompleteAncestryResult {
  // the freshest ast can sometimes be null if the parse was bad.
  if last_good_parse.is_null() {
    return FragmentAutocompleteAncestryResult {
      local_map: DenseHashMap::new(AstName::new()),
      local_stack: Vec::new(),
      ancestry: Vec::new(),
      nearest_statement: null_mut(),
      parent_block: null_mut(),
      fragment_selection_region: Location::new(Position::missing(), Position::missing()),
    };
  }

  let region = unsafe { get_fragment_region_with_block_diff(stale, last_good_parse, &cursor_pos) };
  // SAFETY: stale 有效性由调用方契约保证（C++ 同款非空指针）。
  let ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    unsafe { &mut *stale },
    cursor_pos,
  );

  LUAU_ASSERT!(!ancestry.is_empty());

  // We should only pick up locals that are before the region
  let mut local_map: DenseHashMap<AstName, *mut AstLocal> = DenseHashMap::new(AstName::new());
  let mut local_stack: Vec<*mut AstLocal> = Vec::new();

  for &node in &ancestry {
    // SAFETY: node 取自 ancestry，指向 arena 存活节点；as_ref 对 null 返回 None。
    if let Some(block) = unsafe { ast_node_as::<AstStatBlock>(node).as_ref() } {
      for &stat in block.body.as_slice() {
        // SAFETY: body 元素由解析器保证非空（C++ 同款前提），arena 持有。
        let Some(stat) = (unsafe { stat.as_ref() }) else {
          continue;
        };
        if !(stat.base.location.begin < region.fragment_location.begin) {
          continue;
        }
        // This statement precedes the current one
        if let Some(stat_loc) = ast_node_try_as::<AstStatLocal>(&stat.base) {
          for &v in stat_loc.vars.as_slice() {
            add_local(&mut local_stack, &mut local_map, v);
          }
        } else if let Some(loc_fun) = ast_node_try_as::<AstStatLocalFunction>(&stat.base) {
          add_local(&mut local_stack, &mut local_map, loc_fun.name);
          if loc_fun.base.base.location.contains(cursor_pos) {
            // SAFETY: func 由解析器保证非空（C++ 同款前提），arena 持有。
            if let Some(func) = unsafe { loc_fun.func.as_ref() } {
              for &loc in func.args.as_slice() {
                add_local(&mut local_stack, &mut local_map, loc);
              }
            }
          }
        } else if let Some(glob_fun) = ast_node_try_as::<AstStatFunction>(&stat.base) {
          if glob_fun.base.base.location.contains(cursor_pos) {
            // SAFETY: func 由解析器保证非空（C++ 同款前提），arena 持有。
            if let Some(func) = unsafe { glob_fun.func.as_ref() } {
              // SAFETY: self_ 可空，as_ref 对 null 返回 None（add_local 兜底跳过）。
              add_local(&mut local_stack, &mut local_map, func.self_);

              for &loc in func.args.as_slice() {
                add_local(&mut local_stack, &mut local_map, loc);
              }
            }
          }
        } else if let Some(type_fun) = ast_node_try_as::<AstStatTypeFunction>(&stat.base) {
          if type_fun.base.base.location.contains(cursor_pos) {
            // SAFETY: body 由解析器保证非空（C++ 同款前提），arena 持有。
            if let Some(body) = unsafe { type_fun.body.as_ref() } {
              for &loc in body.args.as_slice() {
                add_local(&mut local_stack, &mut local_map, loc);
              }
            }
          }
        } else if let Some(for_l) = ast_node_try_as::<AstStatFor>(&stat.base) {
          // SAFETY: var 可空，as_ref 对 null 返回 None。
          if let Some(var) = (unsafe { for_l.var.as_ref() })
            && var.location.begin < region.fragment_location.begin
          {
            add_local(&mut local_stack, &mut local_map, for_l.var);
          }
        } else if let Some(for_in) = ast_node_try_as::<AstStatForIn>(&stat.base) {
          for &var in for_in.vars.as_slice() {
            // SAFETY: var 可空，as_ref 对 null 返回 None。
            if let Some(var_ref) = (unsafe { var.as_ref() })
              && var_ref.location.begin < region.fragment_location.begin
            {
              add_local(&mut local_stack, &mut local_map, var);
            }
          }
        } else if let Some(class_decl) = ast_node_try_as::<AstStatClass>(&stat.base) {
          // We need to include the class name as part of the
          // locals so that within the fragment the class name
          // is defined.
          add_local(&mut local_stack, &mut local_map, class_decl.name);
          if class_decl.base.base.location.contains_closed(cursor_pos) {
            let mut current_method: *mut AstExprFunction = null_mut();
            for decl in class_decl.members.as_slice() {
              // CLI-199277: This looks a little weird, like we might end up
              // autocompleting class method arguments in a position like:
              //
              //  class Foobar
              //      function bazbing(alpha, beta, gamma)
              //      end
              //      | -- accidentally include args of bazbing here.
              //  end
              //
              if let Some(method) = decl.get_if_1()
                // SAFETY: method.function 由解析器保证非空；body 可空，
                // as_ref 对 null 返回 None（防御兜底）。
                && (unsafe { (*method.function).body.as_ref() }).is_some_and(|body| {
                  body.base.base.location.begin < cursor_pos
                })
              {
                current_method = method.function;
              }
            }
            // SAFETY: current_method 判非空，arena 持有。
            if let Some(current_method) = unsafe { current_method.as_ref() } {
              for &v in current_method.args.as_slice() {
                add_local(&mut local_stack, &mut local_map, v);
              }
            }
          }
        }
      }
    }

    // SAFETY: node 取自 ancestry，指向 arena 存活节点；as_ref 对 null 返回 None。
    if let Some(expr_func) = (unsafe { ast_node_as::<AstExprFunction>(node).as_ref() })
      && expr_func.base.base.location.contains(cursor_pos)
    {
      for &v in expr_func.args.as_slice() {
        add_local(&mut local_stack, &mut local_map, v);
      }
    }
  }

  FragmentAutocompleteAncestryResult {
    local_map,
    local_stack,
    ancestry,
    nearest_statement: region.nearest_statement,
    parent_block: region.parent_block,
    fragment_selection_region: region.fragment_location,
  }
}
