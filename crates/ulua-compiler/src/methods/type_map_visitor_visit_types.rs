use ulua_ast::{
  records::{ast_stat::AstStat, ast_stat_block::AstStatBlock},
  visit::ast_stat_visit,
};

use crate::{
  methods::type_map_visitor_push_type_aliases::type_map_visitor_push_type_aliases,
  records::type_map_visitor::TypeMapVisitor,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) fn visit_ast_stat_block(this: &mut TypeMapVisitor<'_>, node: *mut AstStatBlock) -> bool {
  unsafe {
    if node.is_null() {
      return false;
    }

    let block = &mut *node;

    let alias_stack_top = type_map_visitor_push_type_aliases(this, node);

    for stat_ptr in block.body.as_slice() {
      let stat: &mut AstStat = &mut **stat_ptr;
      ast_stat_visit(stat, this);
    }

    this.pop_type_aliases(alias_stack_top);
  }

  false
}

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_block(&mut self, node: *mut AstStatBlock) -> bool {
    visit_ast_stat_block(self, node)
  }
}
