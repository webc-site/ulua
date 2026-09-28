use std::ptr::null_mut;

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
/// # Safety
/// 逐参数契约（对照 cpp `getFragmentLocation`，FragmentAutocomplete.cpp）：
/// - `nearest_statement`：可为 null（函数首行即返回空区域）。非 null 时必须指向由
///   fragment AST arena 分配、在本次调用期间存活的 `#[repr(C)]` 节点，且其动态类型属
///   `AstStat` 家族（函数体经 `ast_node_try_as` 按此前提下转）。全程只读，故要求该
///   arena 节点在借用期内无并存可变访问。
/// - `cursor_position`：普通只读借用，调用期间存活即可，无额外约束。
pub unsafe fn get_fragment_location(
  nearest_statement: *mut AstStat,
  cursor_position: &Position,
) -> Location {
  let empty = Location::new(*cursor_position, *cursor_position);

  if nearest_statement.is_null() {
    return empty;
  }

  // Safety: 上方已判空；`nearest_statement` 依函数级契约指向存活的 repr(C) AstStat
  // arena 节点，其 `base`（AstNode）位于偏移 0，转成 `*const AstNode` 后共享解引用，
  // 后续各分支的下转与字段读取全部走安全引用，本函数全程只读。
  let node: &AstNode = unsafe { &*(nearest_statement as *const AstNode) };

  let non_empty = Location::new(node.location.begin, *cursor_position);

  // If your sibling is a do block, do nothing
  if ast_node_try_as::<AstStatBlock>(node).is_some() {
    return empty;
  }

  // Handle AstStatFunction
  if let Some(stat_func) = ast_node_try_as::<AstStatFunction>(node) {
    let func = stat_func.func;
    let name = stat_func.name;
    let local = null_mut();
    // Safety: `func`/`name` 已句柄化为 Node（类型层非空，arena 存活由句柄契约
    // 承载），`as_ptr` 桥交仍按裸指针消费的 `get_function_declaration_extents`；
    // `local` 传 null 对应 cpp 全局函数声明无 local 名的形态。被调函数全程只读。
    let loc = unsafe { get_function_declaration_extents(func.as_ptr(), name.as_ptr(), local) };

    if loc.contains_closed(*cursor_position) {
      return non_empty;
    } else {
      // func 句柄的 `.get()` 即安全只读视图，无裸指针解引用。
      let func_ref = func.get();
      let body_location = func_ref.body.base.base.location;
      if body_location.contains_closed(*cursor_position)
        || stat_func.base.base.location.end <= *cursor_position
      {
        return empty;
      } else if func_ref.base.base.location.contains(*cursor_position) {
        return non_empty;
      }
    }
  }

  // Handle AstStatLocalFunction
  if let Some(stat_local_func) = ast_node_try_as::<AstStatLocalFunction>(node) {
    let func = stat_local_func.func;
    let name = stat_local_func.name;
    let global_func = null_mut();
    // Safety: `func`/`name` 已句柄化为 Node（类型层非空，arena 存活由句柄契约
    // 承载），`as_ptr` 桥交仍按裸指针消费的 `get_function_declaration_extents`；
    // `global_func` 传 null 对应无全局名表达式的形态（cpp 此处 local function 无
    // AstExpr 名）。被调函数对三参全程只读。
    let loc =
      unsafe { get_function_declaration_extents(func.as_ptr(), global_func, name.as_ptr()) };

    if loc.contains_closed(*cursor_position) {
      return non_empty;
    } else {
      // func 句柄的 `.get()` 即安全只读视图，无裸指针解引用。
      let func_ref = func.get();
      let body_location = func_ref.body.base.base.location;
      if body_location.contains_closed(*cursor_position)
        || stat_local_func.base.base.location.end <= *cursor_position
      {
        return empty;
      } else if func_ref.base.base.location.contains(*cursor_position) {
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
    // step 落可空 OptNode（文法上可缺省）：`get()` 即 Option 只读视图，
    // 原 null 折叠的 `as_ref` 门面消失。
    if let Some(step) = stat_for.step.get() {
      let step_location = step.base.location;
      if step_location.contains_closed(*cursor_position) {
        return Location::new(step_location.begin, *cursor_position);
      }
    }
    // to 已句柄化为 Node（parser 必建上界表达式，非空由类型层承载）：
    // `.get()` 即安全只读视图，原判空折叠消失。
    {
      let to = stat_for.to.get();
      let to_location = to.base.location;
      if to_location.contains_closed(*cursor_position) {
        return Location::new(to_location.begin, *cursor_position);
      }
    }
    // from 已句柄化为 Node（parser 必建下界表达式），同上。
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
  let nearest_if_ptr = get_nearest_if_to_cursor(nearest_statement, cursor_position);
  // Safety: `get_nearest_if_to_cursor` 是安全函数，返回 null 或指向 arena 存活节点的
  // `*mut AstStatIf`；`as_ref()` 将 null 折叠为 `None`，非 null 时按契约指向存活 AST 节点
  // （arena 节点地址不移动，函数借用覆盖后续使用点），仅生成只读引用。
  let nearest_if_opt = unsafe { nearest_if_ptr.as_ref() };
  if let Some(if_stmt) = nearest_if_opt {
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
