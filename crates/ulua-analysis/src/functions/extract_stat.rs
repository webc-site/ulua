use ulua_ast::{
  records::{ast_node::AstNode, ast_stat_block::AstStatBlock, ast_stat_error::AstStatError},
  rtti::{AstNodeClass, ast_node_is, ast_node_try_as},
};

use crate::{functions::is_identifier::is_identifier, records::arena_handle::alias_opt};

/// 从 `ancestry` 尾部（node / parent / grand / great-grand 四层）逐层探测类型为
/// `T` 的语句节点，命中即返回其共享借用。
///
/// `ancestry` 为分析期存活、由 AST arena 持有的节点裸指针切片（元素可为 null），
/// 判空与下转收在本函数内，返回 `Option<&T>` 而非可空裸指针——对应 cpp 的
/// `node->as<T>()` 命中/未命中语义（未命中即 `None`，见 `extractStat`，
/// AutocompleteCore.cpp:1428-1442）。
pub fn extract_stat<T: AstNodeClass>(ancestry: &[*mut AstNode]) -> Option<&'static T> {
  // 四层槽位统一用迭代器取位（cpp `rbegin()` 下标的迭代器读法），可空裸指针
  // 门面把「槽位缺失」与「null 槽位」一并折成 Option 早退，判空从内部消失。
  let node = alias_opt(ancestry.iter().copied().last()?)?;
  if let Some(t) = ast_node_try_as::<T>(node) {
    return Some(t);
  }

  let parent = alias_opt(ancestry.iter().copied().nth_back(1)?)?;
  let grand_parent = alias_opt(ancestry.iter().copied().nth_back(2)?)?;

  if let Some(t_parent) = ast_node_try_as::<T>(parent)
    && ast_node_is::<AstStatBlock>(grand_parent)
  {
    return Some(t_parent);
  }

  // 第四层仅在前三层落空后才参与判定（原实现先判 parent 命中再取 great-grand），
  // 故此处才做本层的 Option 早退。
  let great_grand_parent = alias_opt(ancestry.iter().copied().nth_back(3)?)?;

  if let Some(t_great) = ast_node_try_as::<T>(great_grand_parent)
    && ast_node_is::<AstStatBlock>(grand_parent)
    && ast_node_is::<AstStatError>(parent)
    && is_identifier(node)
  {
    return Some(t_great);
  }

  None
}
