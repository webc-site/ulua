use alloc::collections::BTreeMap;

use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_stat_error::AstStatError, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_if::AstStatIf, ast_stat_repeat::AstStatRepeat, ast_stat_while::AstStatWhile,
    position::Position,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
use ulua_common::fflag;

use crate::{
  enums::{autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind},
  functions::{
    autocomplete_keywords::{autocomplete_keywords, insert_keyword},
    get_paren_recommendation::get_paren_recommendation,
    is_binding_legal_at_current_position::is_binding_legal_at_current_position,
    is_identifier::is_identifier,
    is_in_local_names::is_in_local_names,
    is_valid_break_continue_context::is_valid_break_continue_context,
    to_string_symbol::to_string_symbol,
  },
  records::{
    autocomplete_entry::AutocompleteEntry, module::Module, scope::Scope,
    scope_registry::resolve_scope,
  },
  type_aliases::{autocomplete_entry_map::AutocompleteEntryMap, scope_ptr_type::ScopePtr},
};

/// 三档语句关键字表共有的基础集（顺序同 C++）。
const K_STATEMENT_KEYWORDS_BASE: [&str; 12] = [
  "while", "if", "local", "repeat", "function", "do", "for", "return", "break", "continue", "type",
  "export",
];
/// LuauAutocompleteConst 档追加项。
const K_STATEMENT_KEYWORDS_CONST_EXTRA: [&str; 1] = ["const"];
/// 导出语法档追加项（C++ 原表尾部重复 "export"，map 插入幂等故省略）。
const K_STATEMENT_KEYWORDS_EXPORT_EXTRA: [&str; 2] = ["const", "export"];

pub fn autocomplete_statement(
  _module: &Module,
  ancestry: &[*mut AstNode],
  scope_at_position: &ScopePtr,
  position: &mut Position,
) -> AutocompleteEntryMap {
  let mut result: AutocompleteEntryMap = BTreeMap::new();

  if is_in_local_names(ancestry, *position) {
    autocomplete_keywords(ancestry, *position, &mut result);
    return result;
  }

  let mut scope: Option<&Scope> = Some(scope_at_position.as_ref());
  while let Some(scope_ref) = scope {
    for (name, binding) in &scope_ref.bindings {
      if !is_binding_legal_at_current_position(name, binding, *position) {
        continue;
      }

      let n = to_string_symbol(name);
      if !result.contains_key(&n) {
        result.insert(
          n.clone(),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::Binding,
            r#type: Some(binding.type_id),
            deprecated: binding.deprecated,
            wrong_index_type: false,
            type_correct: TypeCorrectKind::None,
            containing_extern_type: None,
            prop: None,
            documentation_symbol: binding.documentation_symbol.clone(),
            tags: Default::default(),
            parens: get_paren_recommendation(binding.type_id, ancestry, TypeCorrectKind::None),
            insert_text: None,
            indexed_with_self: false,
          },
        );
      }
    }

    scope = scope_ref.parent.and_then(resolve_scope);
  }

  let should_include_break_and_continue = is_valid_break_continue_context(ancestry, *position);

  // 三档关键字表仅在追加项上不同，遍历顺序与 C++ 一致。
  let (base, extra): (&[&str], &[&str]) =
    if fflag::LuauExportValueSyntax.get() && fflag::LuauAutocompleteExport.get() {
      (
        &K_STATEMENT_KEYWORDS_BASE,
        &K_STATEMENT_KEYWORDS_EXPORT_EXTRA,
      )
    } else if fflag::LuauAutocompleteConst.get() {
      (
        &K_STATEMENT_KEYWORDS_BASE,
        &K_STATEMENT_KEYWORDS_CONST_EXTRA,
      )
    } else {
      (&K_STATEMENT_KEYWORDS_BASE, &[])
    };
  for &kw in base.iter().chain(extra.iter()) {
    if (kw != "break" && kw != "continue") || should_include_break_and_continue {
      insert_keyword(&mut result, kw);
    }
  }

  for &ast_node in ancestry.iter().rev() {
    // Safety: `ancestry` 由补全入口沿光标路径构造，每个元素都是解析 arena 持有的
    // 存活非空 AstNode 指针（C++ `ancestry.rbegin()` 同款前提），遍历期间只读。
    let node_ref = unsafe { &*ast_node };

    if let Some(stat_for_in) = ast_node_try_as::<AstStatForIn>(node_ref) {
      // body 已句柄化为非空 Node（parseForIn 先建 body 块）：is_null 防御短路
      // 与死 unsafe 随类型消失，`.get()` 仅读 has_end 布尔位。
      if !stat_for_in.body.get().has_end {
        insert_keyword(&mut result, "end");
      }
    } else if let Some(stat_for) = ast_node_try_as::<AstStatFor>(node_ref) {
      // body 已句柄化为 Node：`.get()` 即安全只读视图（C++ 此处无条件
      // `statFor->body->hasEnd`），死 unsafe 消解。
      if !stat_for.body.get().has_end {
        insert_keyword(&mut result, "end");
      }
    } else if let Some(stat_if) = ast_node_try_as::<AstStatIf>(node_ref) {
      let mut has_end = stat_if.thenbody.has_end;
      if let Some(else_stat) = stat_if.elsebody.get()
        && let Some(else_block) = ast_node_try_as::<AstStatBlock>(&else_stat.base)
      {
        has_end = else_block.has_end;
      }
      if !has_end {
        insert_keyword(&mut result, "end");
      }
    } else if let Some(stat_while) = ast_node_try_as::<AstStatWhile>(node_ref) {
      if !stat_while.body.has_end {
        insert_keyword(&mut result, "end");
      }
    } else if let Some(expr_function) = ast_node_try_as::<AstExprFunction>(node_ref)
      && !expr_function.body.has_end
    {
      insert_keyword(&mut result, "end");
    }

    if let Some(expr_block) = ast_node_try_as::<AstStatBlock>(node_ref)
      && !expr_block.has_end
    {
      insert_keyword(&mut result, "end");
    }
  }

  if ancestry.len() >= 2 {
    let parent = ancestry[ancestry.len() - 2];
    // Safety: 长度守卫对应 C++ `ancestry.rbegin()[1]`，ancestry 元素按构造不变量
    // 非空且指向 arena 存活节点，本函数只读该树。
    let parent_ref = unsafe { &*parent };
    if let Some(stat_if) = ast_node_try_as::<AstStatIf>(parent_ref) {
      let elsebody = stat_if.elsebody;
      let else_location = stat_if.else_location;
      if elsebody.is_null() || else_location.is_some_and(|loc| loc.contains_closed(*position)) {
        insert_keyword(&mut result, "else");
        insert_keyword(&mut result, "elseif");
      }
    }

    if let Some(stat_repeat) = ast_node_try_as::<AstStatRepeat>(parent_ref)
      // body 已句柄化为 Node：`.get()` 即安全只读视图（C++ `statRepeat->body->hasEnd`
      // 亦无判空），原 unsafe 裸指针解引用消失。
      && !stat_repeat.body.get().has_end
    {
      insert_keyword(&mut result, "until");
    }
  }

  // 对应 cpp `AutocompleteCore.cpp:1567-1577`：`ancestry.rbegin()` 逆序四层
  // `iter[3]…iter[0]`（`statIf && !statIf->elsebody && iter[2]->is<AstStatBlock>()
  // && iter[1]->is<AstStatError>() && isIdentifier(iter[0])`）。索引读法
  // `ancestry[len - k]` 改为迭代器 `nth_back`/`last`，长度守卫折进 Option 元组
  // 模式；各层 null 槽位在调用点即被 `as_ref` 折成 `Option` 挡掉，
  // `is_identifier` 只收非空 `&AstNode`。
  // Safety: ancestry 槽位为补全入口沿光标路径收集的 arena 存活节点指针或 null；
  // `as_ref` 对 null 返回 None，非空者化为共享只读借用，仅交给只读的 try_as /
  // ast_node_is / is_identifier 读取类索引与字段。
  if let (Some(iter3), Some(iter2), Some(iter1), Some(iter0)) = (
    ancestry.iter().copied().nth_back(3),
    ancestry.iter().copied().nth_back(2),
    ancestry.iter().copied().nth_back(1),
    ancestry.iter().copied().last(),
  ) {
    let stat_if_ok = unsafe { iter3.as_ref() }
      .and_then(|n| ast_node_try_as::<AstStatIf>(n))
      .is_some_and(|stat_if| stat_if.elsebody.is_null());
    if stat_if_ok
      && unsafe { iter2.as_ref() }.is_some_and(ast_node_is::<AstStatBlock>)
      && unsafe { iter1.as_ref() }.is_some_and(ast_node_is::<AstStatError>)
      && unsafe { iter0.as_ref() }.is_some_and(is_identifier)
    {
      insert_keyword(&mut result, "else");
      insert_keyword(&mut result, "elseif");
    }
  }

  // extractStat<AstStatRepeat>(ancestry) isn't available here via required context,
  // so mimic by scanning for the first AstStatRepeat from end.
  let mut found_repeat: Option<&AstStatRepeat> = None;
  for &node in ancestry.iter().rev() {
    // Safety: 逆序遍历的每个 ancestry 元素都是 arena 存活节点指针（补全入口按
    // 光标路径收集），裸化为共享引用后仅交给只读的 try_as 做 class_index 比较。
    if let Some(stat_repeat) = ast_node_try_as::<AstStatRepeat>(unsafe { &*node }) {
      found_repeat = Some(stat_repeat);
      break;
    }
  }
  if let Some(stat_repeat) = found_repeat
    // body 已句柄化为 Node：`.get()` 即安全只读视图（C++ extractStat 后同样
    // 无条件 `->body->hasEnd`），原 unsafe 裸指针解引用消失。
    && !stat_repeat.body.get().has_end
  {
    insert_keyword(&mut result, "until");
  }

  result
}
