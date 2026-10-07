use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_while::AstStatWhile,
    location::Location, position::Position,
  },
  rtti::ast_node_try_as,
};

use crate::functions::{
  get_function_declaration_extents::get_function_declaration_extents,
  get_nearest_if_to_cursor::get_nearest_if_to_cursor,
};

/// 对应 cpp `getFragmentLocation`（FragmentAutocomplete.cpp）。
/// - `nearest_statement`：可为 None（函数首行即返回空区域）。
/// - `cursor_position`：普通只读借用。
pub fn get_fragment_location(
  nearest_statement: Option<&AstStat>,
  cursor_position: &Position,
) -> Location {
  let empty = Location::new(*cursor_position, *cursor_position);

  let Some(stat) = nearest_statement else {
    return empty;
  };

  let node: &AstNode = &stat.base;
  let non_empty = Location::new(node.location.begin, *cursor_position);

  // If your sibling is a do block, do nothing
  if ast_node_try_as::<AstStatBlock>(node).is_some() {
    return empty;
  }

  // Handle AstStatFunction
  if let Some(stat_func) = ast_node_try_as::<AstStatFunction>(node) {
    let func = stat_func.func.get();
    let name = stat_func.name.get();
    let loc = get_function_declaration_extents(func, Some(name), None);

    if loc.contains_closed(*cursor_position) {
      return non_empty;
    } else {
      let body_location = func.body.base.base.location;
      if body_location.contains_closed(*cursor_position)
        || stat_func.base.base.location.end <= *cursor_position
      {
        return empty;
      } else if func.base.base.location.contains(*cursor_position) {
        return non_empty;
      }
    }
  }

  // Handle AstStatLocalFunction
  if let Some(stat_local_func) = ast_node_try_as::<AstStatLocalFunction>(node) {
    let func = stat_local_func.func.get();
    let name = stat_local_func.name.get();
    let loc = get_function_declaration_extents(func, None, Some(name));

    if loc.contains_closed(*cursor_position) {
      return non_empty;
    } else {
      let body_location = func.body.base.base.location;
      if body_location.contains_closed(*cursor_position)
        || stat_local_func.base.base.location.end <= *cursor_position
      {
        return empty;
      } else if func.base.base.location.contains(*cursor_position) {
        return non_empty;
      }
    }
  }

  // Handle AstStatWhile
  if let Some(stat_while) = ast_node_try_as::<AstStatWhile>(node) {
    if !stat_while.has_do {
      return non_empty;
    } else {
      return empty;
    }
  }

  // Handle AstStatFor
  if let Some(stat_for) = ast_node_try_as::<AstStatFor>(node) {
    if let Some(step) = stat_for.step.get() {
      let step_location = step.base.location;
      if step_location.contains_closed(*cursor_position) {
        return Location::new(step_location.begin, *cursor_position);
      }
    }
    {
      let to = stat_for.to.get();
      let to_location = to.base.location;
      if to_location.contains_closed(*cursor_position) {
        return Location::new(to_location.begin, *cursor_position);
      }
    }
    {
      let from = stat_for.from.get();
      let from_location = from.base.location;
      if from_location.contains_closed(*cursor_position) {
        return Location::new(from_location.begin, *cursor_position);
      }
    }

    if !stat_for.has_do {
      return non_empty;
    } else {
      let completeable_extents = Location::new(
        stat_for.base.base.location.begin,
        stat_for.do_location.begin,
      );
      if completeable_extents.contains_closed(*cursor_position) {
        return non_empty;
      }
      return empty;
    }
  }

  // Handle AstStatForIn
  if let Some(stat_for_in) = ast_node_try_as::<AstStatForIn>(node) {
    if !stat_for_in.has_do {
      return non_empty;
    } else {
      let completeable_extents = Location::new(
        stat_for_in.base.base.location.begin,
        stat_for_in.do_location.begin,
      );
      if completeable_extents.contains_closed(*cursor_position) {
        if !stat_for_in.has_in {
          return non_empty;
        } else {
          // [for ... in ... do] - the cursor can either be between [for ... in] or [in ... do]
          if *cursor_position < stat_for_in.in_location.begin {
            return non_empty;
          } else {
            return Location::new(stat_for_in.in_location.begin, *cursor_position);
          }
        }
      }
      return empty;
    }
  }

  // Handle AstStatIf
  if let Some(if_stmt) = get_nearest_if_to_cursor(Some(stat), cursor_position) {
    let cond_loc = if_stmt.condition.base.location;
    let condition_extents = Location::new(cond_loc.begin, cond_loc.end);

    if condition_extents.contains_closed(*cursor_position) || if_stmt.then_location.is_none() {
      // CLI-152249 - the condition parse location can sometimes be after the body of the if
      // statement. This is a bug that results returning locations like {3,0 - 2,0} which is
      // wrong.
      if cond_loc.begin > *cursor_position {
        return empty;
      }
      return Location::new(cond_loc.begin, *cursor_position);
    } else if if_stmt
      .thenbody
      .base
      .base
      .location
      .contains_closed(*cursor_position)
    {
      return empty;
    } else if let Some(else_body) = if_stmt.elsebody.get() {
      if let Some(else_if) = ast_node_try_as::<AstStatIf>(&else_body.base) {
        let cond_ref = else_if.condition.get();
        let else_if_condition_extents =
          Location::new(else_if.base.base.location.begin, cond_ref.base.location.end);
        if else_if_condition_extents.contains_closed(*cursor_position) {
          return Location::new(cond_ref.base.location.begin, *cursor_position);
        }
        let thenbody_has_end = else_if.thenbody.has_end;
        if thenbody_has_end {
          return empty;
        } else {
          return Location::new(else_body.base.location.begin, *cursor_position);
        }
      }
      return empty;
    }
  }

  non_empty
}
