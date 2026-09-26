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
  rtti::{ast_node_try_as, ast_node_try_as_ptr},
};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  functions::{
    find_ancestry_at_position_for_autocomplete_ast_query::find_ancestry_at_position_for_autocomplete_ast_stat_block_position,
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
  // Safety: `local` 是 parser arena 内 AstLocal 的裸指针（来自语句/函数节点的
  // AstArray 字段），由调用方以 add_local 收集；`as_ref` 对 null 直接给 `None` 不
  // 解引用（防御解析中断产物），Some 分支只读 `name` 标量，指针随后按 C++ 同款语义
  // 存入栈/映射，与 arena 共生命周期。
  let Some(local_ref) = (unsafe { local.as_ref() }) else {
    return;
  };
  local_stack.push(local);
  *local_map.get_or_insert(local_ref.name) = local;
}

/// 对照 C++ `findAncestryForFragmentParse`（FragmentAutocomplete.cpp:387）。
///
/// # Safety
/// - `stale`：上一次解析持有树根部的非空 `*mut AstStatBlock`（C++ 形参 `stale`，
///   即 Frontend 双树轮转中的旧 root）。调用期内该树必须存活且不被别名可变借用：
///   函数内部以 `&mut *stale` 走祖先查询、以 `&mut *stale` 访问 block diff，且返回
///   结果的 ancestry / nearest_statement / parent_block 裸指针全部指向这棵树。
/// - `cursor_pos`：按值传入的光标坐标，无指针前提；允许落在两棵树的任意位置。
/// - `last_good_parse`：最近一次完整解析的树根，可为 null（对应 C++ 对
///   `lastGoodParse == nullptr` 提前返回空结果，本函数同样短路）；非空时须与
///   `stale` 同属解析 arena 且调用期间存活，内部会完整遍历它做 block diff。
pub unsafe fn find_ancestry_for_fragment_parse(
  stale: *mut AstStatBlock,
  cursor_pos: Position,
  last_good_parse: *mut AstStatBlock,
) -> FragmentAutocompleteAncestryResult {
  // the freshest ast can sometimes be null if the parse was bad.
  if last_good_parse.is_null() {
    return FragmentAutocompleteAncestryResult {
      local_map: DenseHashMap::default(),
      local_stack: Vec::new(),
      ancestry: Vec::new(),
      nearest_statement: null_mut(),
      parent_block: null_mut(),
      fragment_selection_region: Location::new(Position::missing(), Position::missing()),
    };
  }

  // Safety: 满足 `get_fragment_region_with_block_diff` 对两个块指针的前提——
  // `last_good_parse` 已在函数入口判空（null 时提前返回空结果），`stale` 非空且
  // 两棵树在调用期间存活由本函数 `# Safety` 契约保证；期间无别名可变访问。
  let region = unsafe { get_fragment_region_with_block_diff(stale, last_good_parse, &cursor_pos) };
  // Safety: `&mut *stale` 依函数契约合法（非空、独占可借用）；被调的祖先查询沿
  // 位置路径下行收集节点，语义与 C++ `findAncestryAtPositionForAutocomplete(stale,…)`
  // 的非 const visit 相同，不改动树结构。
  let ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    unsafe { &mut *stale },
    cursor_pos,
  );

  LUAU_ASSERT!(!ancestry.is_empty());

  // We should only pick up locals that are before the region
  let mut local_map: DenseHashMap<AstName, *mut AstLocal> = DenseHashMap::default();
  let mut local_stack: Vec<*mut AstLocal> = Vec::new();

  for &node in &ancestry {
    // Safety: `node` 来自上方刚沿 `stale` 树收集的 ancestry，指向 arena 存活节点；
    // `ast_node_try_as_ptr` 类位不命中 AstStatBlock 时返回 None，命中后提供块引用。
    if let Some(block) = unsafe { ast_node_try_as_ptr::<AstStatBlock>(node) } {
      for stat in block.body.iter() {
        if !(stat.base.location.begin < region.fragment_location.begin) {
          continue;
        }
        // This statement precedes the current one
        if let Some(stat_loc) = ast_node_try_as::<AstStatLocal>(&stat.base) {
          for &v in stat_loc.vars.as_slice() {
            add_local(&mut local_stack, &mut local_map, v);
          }
        } else if let Some(loc_fun) = ast_node_try_as::<AstStatLocalFunction>(&stat.base) {
          add_local(&mut local_stack, &mut local_map, loc_fun.name.as_ptr());
          if loc_fun.base.base.location.contains(cursor_pos) {
            // `func` 已句柄化为 Node<AstExprFunction>（parser 必建非空，Ast.h 构造
            // 端论证见 records 注释），`.get()` 给存活引用后直取 args。
            for loc in loc_fun.func.get().args.iter_nodes() {
              add_local(&mut local_stack, &mut local_map, loc.as_ptr());
            }
          }
        } else if let Some(glob_fun) = ast_node_try_as::<AstStatFunction>(&stat.base) {
          if glob_fun.base.base.location.contains(cursor_pos) {
            // func 已句柄化为 Node<AstExprFunction>（parser 必填非空，records 注释
            // 载论证），`.get()` 直出存活引用；self_ 对普通全局函数仍可为 null，
            // 交由 add_local 的判空分支跳过。
            let func = glob_fun.func.get();
            add_local(&mut local_stack, &mut local_map, func.self_.as_ptr());

            for loc in func.args.iter_nodes() {
              add_local(&mut local_stack, &mut local_map, loc.as_ptr());
            }
          }
        } else if let Some(type_fun) = ast_node_try_as::<AstStatTypeFunction>(&stat.base) {
          if type_fun.base.base.location.contains(cursor_pos) {
            // Safety: type function 语句的 `body` 块由 parser 建节点时写入非空
            // （C++ `typeFun->body->args` 无条件解引用），节点随 arena 存活。
            if let Some(body) = unsafe { type_fun.body.as_ref() } {
              for loc in body.args.iter_nodes() {
                add_local(&mut local_stack, &mut local_map, loc.as_ptr());
              }
            }
          }
        } else if let Some(for_l) = ast_node_try_as::<AstStatFor>(&stat.base) {
          // var 已句柄化为 Node（push_local alloc 恒非空，中断解析亦交回 arena
          // 节点），`.get()` 即安全只读视图，原 null 短路门面消失；add_local
          // 仍按指针身份登记，as_ptr 桥交。
          if for_l.var.get().location.begin < region.fragment_location.begin {
            add_local(&mut local_stack, &mut local_map, for_l.var.as_ptr());
          }
        } else if let Some(for_in) = ast_node_try_as::<AstStatForIn>(&stat.base) {
          for &var in for_in.vars.as_slice() {
            // Safety: `vars` 数组元素指向 arena 的 AstLocal；null 经 `as_ref` 折为
            // `None` 后跳过比较分支，非空时仅读 location 标量字段。
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
                && let Some(func) = unsafe { method.function.as_ref() }
                && func.body.base.base.location.begin < cursor_pos
              {
                current_method = method.function;
              }
            }
            // Safety: `current_method` 初值 null、仅被上方循环里的非空
            // `method.function` 覆写，两种取值 `as_ref` 都能安全折叠；指针对象
            // 活在 `stale` 树的 arena 中。
            if let Some(current_method) = unsafe { current_method.as_ref() } {
              for v in current_method.args.iter_nodes() {
                add_local(&mut local_stack, &mut local_map, v.as_ptr());
              }
            }
          }
        }
      }
    }

    // Safety: 第二轮复用同一 `ancestry` 元素——仍是 `stale` 树 arena 内的存活节点；
    // class_index 非 AstExprFunction 时 `ast_node_try_as_ptr` 产出 None。
    if let Some(expr_func) = (unsafe { ast_node_try_as_ptr::<AstExprFunction>(node) })
      && expr_func.base.base.location.contains(cursor_pos)
    {
      for v in expr_func.args.iter_nodes() {
        add_local(&mut local_stack, &mut local_map, v.as_ptr());
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
