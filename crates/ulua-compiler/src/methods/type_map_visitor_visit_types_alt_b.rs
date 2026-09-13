use ulua_ast::{
  records::{ast_stat::AstStat, ast_stat_repeat::AstStatRepeat},
  visit::{ast_expr_visit, ast_stat_visit},
};

use crate::{
  methods::type_map_visitor_push_type_aliases::type_map_visitor_push_type_aliases,
  records::type_map_visitor::TypeMapVisitor,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) fn visit_ast_stat_repeat(
  this: &mut TypeMapVisitor<'_>,
  node: *mut AstStatRepeat,
) -> bool {
  unsafe {
    if node.is_null() {
      return false;
    }

    let repeat = &mut *node;

    let alias_stack_top = type_map_visitor_push_type_aliases(this, repeat.body);

    for stat_ptr in (*repeat.body).body.as_slice() {
      let stat: &mut AstStat = &mut **stat_ptr;
      ast_stat_visit(stat, this);
    }

    if !repeat.condition.is_null() {
      ast_expr_visit(repeat.condition, this);
    }

    this.pop_type_aliases(alias_stack_top);
  }

  false
}

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_repeat(&mut self, node: *mut AstStatRepeat) -> bool {
    visit_ast_stat_repeat(self, node)
  }
}
