use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_if::AstStatIf,
  },
  rtti::{ast_node_is, ast_node_try_as_ptr},
};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `node` 非空、对齐，指向 parse arena 中存活至本次 check 结束的 `AstStat`；本函数沿
  /// block/if 子节点递归只读判型（`elsebody` 已判空后进入），单线程串行、无并发可变别名。
  /// cpp `Analysis/src/TypeChecker2.cpp:381`（`bool TypeChecker2::hasBreak(AstStat*)`）。
  pub unsafe fn type_checker_2_has_break(&mut self, node: *mut AstStat) -> bool {
    // cpp TypeChecker2.cpp:381
    // Safety: `node` 由 AST 遍历/入口保证指向 arena 存活且对齐的 AstStat 节点；
    // `as *mut AstNode` 是 repr(C) 单继承基址重合视图。ast_node_try_as_ptr 以
    // RTTI class index 判型，未命中返回 None，命中即类型正确且引用随遍历存活。
    if let Some(block) = unsafe { ast_node_try_as_ptr::<AstStatBlock>(node) } {
      return block
        .body
        .iter_nodes()
        .any(|stat| unsafe { self.type_checker_2_has_break(stat.as_ptr()) });
    }

    // Safety: `node` 同上为存活对齐 AST 节点，只读其 base.class_index 比对。
    if ast_node_is::<AstStatBreak>(unsafe { &*node }) {
      return true;
    }

    // Safety: `node` 存活，RTTI class index 命中才返回 Some(&AstStatIf)。
    if let Some(if_stat) = unsafe { ast_node_try_as_ptr::<AstStatIf>(node) } {
      return unsafe { self.type_checker_2_has_break(if_stat.thenbody.as_ptr().cast::<AstStat>()) }
        || (if_stat.elsebody.is_some()
          && unsafe { self.type_checker_2_has_break(if_stat.elsebody.as_ptr()) });
    }

    false
  }
}
