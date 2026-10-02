use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_local::AstLocal, ast_name::AstName,
    ast_stat_block::AstStatBlock, ast_stat_class::AstStatClass, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    ast_stat_type_function::AstStatTypeFunction, location::Location, node_handle::OptNode,
    position::Position,
  },
  rtti::ast_node_try_as,
};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  functions::{
    find_ancestry_at_position_for_autocomplete_ast_query::find_ancestry_at_position_for_autocomplete_ast_stat_block_position,
    get_fragment_region_with_block_diff::get_fragment_region_with_block_diff,
  },
  records::{
    arena_handle::{Handle, alias_opt},
    fragment_autocomplete_ancestry_result::FragmentAutocompleteAncestryResult,
  },
};

/// 对照 C++ `localStack.push_back(v); localMap[v->name] = v;`：
/// 登记局部变量 → 压栈 + 名字映射；null 跳过（解析器保证非空，防御兜底）。
fn add_local(
  local_stack: &mut Vec<*mut AstLocal>,
  local_map: &mut DenseHashMap<AstName, *mut AstLocal>,
  local: *mut AstLocal,
) {
  let Some(local_ref) = alias_opt(local) else {
    return;
  };
  local_stack.push(local);
  *local_map.get_or_insert(local_ref.name) = local;
}

/// 对照 C++ `findAncestryForFragmentParse`（FragmentAutocomplete.cpp:387）。
///
/// §2：入口已句柄化——`stale` 为双树轮转中的旧 root（C++ 形参 `stale`，
/// 仅在 `last_good_parse` 非空的路径上被祖先查询/block diff 解引用，nullptr
/// 于 cpp 属 UB，此处保留 `Option` 直至解引用点收口为确定性 panic）；
/// `cursor_pos` 按值传入的光标坐标，允许落在两棵树的任意位置；
/// `last_good_parse` 最近一次完整解析的树根，可为 `None`（对应 C++ 对
/// `lastGoodParse == nullptr` 提前返回空结果，本函数同样短路）；两棵树在
/// 调用期间存活由 `Handle` 模块级 arena 保活契约承载，返回值中的
/// ancestry / nearest_statement / parent_block 皆指向这两棵树。
pub fn find_ancestry_for_fragment_parse(
  stale: Option<Handle<AstStatBlock>>,
  cursor_pos: Position,
  last_good_parse: Option<Handle<AstStatBlock>>,
) -> FragmentAutocompleteAncestryResult {
  // the freshest ast can sometimes be null if the parse was bad.
  let fresh = match last_good_parse {
    Some(fresh) => fresh,
    None => {
      return FragmentAutocompleteAncestryResult {
        local_map: DenseHashMap::default(),
        local_stack: Vec::new(),
        ancestry: Vec::new(),
        nearest_statement: None,
        parent_block: None,
        fragment_selection_region: Location::new(Position::missing(), Position::missing()),
      };
    }
  };

  let region = get_fragment_region_with_block_diff(stale, fresh, &cursor_pos);
  let stale_root =
    stale.expect("stale 根在 lastGoodParse 非空时应在场（cpp findAncestry 直解引用 UB 收口）");
  let ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    stale_root.get_mut(),
    cursor_pos,
  );

  LUAU_ASSERT!(!ancestry.is_empty());

  // We should only pick up locals that are before the region
  let mut local_map: DenseHashMap<AstName, *mut AstLocal> = DenseHashMap::default();
  let mut local_stack: Vec<*mut AstLocal> = Vec::new();

  for &node in &ancestry {
    // `node` 来自上方刚沿 `stale` 树收集的 ancestry（arena 存活节点或 null）：
    // 经句柄门面 `OptNode::from_ptr` 折叠可空性，判型下转走句柄上生命周期
    // 正确的 `try_as`，借用半径由本轮局部句柄供给，不再锻造假 'static。
    let node_handle = OptNode::from_ptr(node);
    if let Some(block) = node_handle.try_as::<AstStatBlock>() {
      for stat in block.body.iter() {
        // This statement precedes the current one
        if stat.base.location.begin >= region.fragment_location.begin {
          continue;
        }
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
          if type_fun.base.base.location.contains(cursor_pos)
            && let Some(body) = alias_opt(type_fun.body)
          {
            for loc in body.args.iter_nodes() {
              add_local(&mut local_stack, &mut local_map, loc.as_ptr());
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
            if let Some(var_ref) = alias_opt(var)
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
            // §2：可空槽直接 `Option<&'static AstExprFunction>`（alias_opt 的 arena
            // 只读视图），null_mut 哨兵消失。循环逐字保留「最后命中覆写前值」的
            // cpp currentMethod 语义——命中项互异，无比较子并列语义可反转。
            let mut current_method: Option<&AstExprFunction> = None;
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
                && let Some(func) = alias_opt(method.function)
                && func.body.base.base.location.begin < cursor_pos
              {
                current_method = Some(func);
              }
            }
            if let Some(current_method) = current_method {
              for v in current_method.args.iter_nodes() {
                add_local(&mut local_stack, &mut local_map, v.as_ptr());
              }
            }
          }
        }
      }
    }

    // 同一 `node_handle` 复用：仍是本轮局部句柄供给借用，类位非
    // AstExprFunction 时 `try_as` 产出 None。
    if let Some(expr_func) = node_handle.try_as::<AstExprFunction>()
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
