use ulua_ast::records::ast_stat_if::AstStatIf;

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  functions::matches::matches,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, push_scope::PushScope},
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `i` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_if(&mut self, i: *mut AstStatIf) -> ControlFlow {
    let i = unsafe { &*i };
    self.visit_expr_ast_expr(i.condition);

    let then_scope = self.make_child_scope(ScopeType::Linear);
    let else_scope = self.make_child_scope(ScopeType::Linear);

    let then_cf = {
      let mut ps = PushScope::new(&mut self.scope_stack, then_scope);
      let cf = self.visit_ast_stat_block(i.thenbody);
      ps.pop();
      cf
    };

    let mut else_cf = ControlFlow::None;
    if !i.elsebody.is_null() {
      let mut ps = PushScope::new(&mut self.scope_stack, else_scope);
      else_cf = self.visit_ast_stat(i.elsebody);
      ps.pop();
    }

    let scope = self.current_scope();
    if then_cf != ControlFlow::None && else_cf == ControlFlow::None {
      unsafe { (*scope).inherit(else_scope) };
    } else if then_cf == ControlFlow::None && else_cf != ControlFlow::None {
      unsafe { (*scope).inherit(then_scope) };
    } else if (then_cf as u32 | else_cf as u32) == ControlFlow::None as u32 {
      unsafe { self.join(scope, then_scope, else_scope) };
    }

    if then_cf == else_cf {
      then_cf
    } else if matches(then_cf, ControlFlow::Returns)
      || matches(then_cf, ControlFlow::Throws)
        && (matches(else_cf, ControlFlow::Returns) || matches(else_cf, ControlFlow::Throws))
    {
      ControlFlow::Returns
    } else {
      ControlFlow::None
    }
  }
}
