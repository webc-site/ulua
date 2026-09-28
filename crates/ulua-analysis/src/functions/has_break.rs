use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_if::AstStatIf,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as_ptr},
};
pub(crate) fn has_break(node: *mut AstStat) -> bool {
  if node.is_null() {
    return false;
  }

  // Safety: `node` 已在函数头 `is_null()` 守卫为非空；`ast_node_try_as_ptr` 按
  // class-index 下转，null/未命中返回 None 且从不解引用，命中即按 repr(C) 基址
  // 重合借出完整存活节点的只读引用（AST 节点存活于 arena，bump 分配、块地址不
  // 移动）。`body`/`thenbody` 子槽已句柄化（`Node` 非空由类型端兑现，`.elsebody`
  // 经 `is_none()` 守卫），子指针仅喂回本函数既有 `*mut` 递归门面。
  // 全程单线程只读遍历，无别名冲突。
  unsafe {
    if let Some(stat) = ast_node_try_as_ptr::<AstStatBlock>(node) {
      // 任一子语句含 break 即可提前返回
      for child in stat.body.iter_nodes() {
        if has_break(child.as_ptr()) {
          return true;
        }
      }
      return false;
    }

    if ast_node_is_ptr::<AstStatBreak>(node) {
      return true;
    }

    if let Some(stat) = ast_node_try_as_ptr::<AstStatIf>(node) {
      if has_break(stat.thenbody.as_ptr().cast::<AstStat>()) {
        return true;
      }

      if stat.elsebody.is_some() && has_break(stat.elsebody.as_ptr()) {
        return true;
      }

      return false;
    }

    false
  }
}
