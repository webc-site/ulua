use ulua_ast::records::ast_stat_for_in::AstStatForIn;

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, dfg_scope::DfgScope, push_scope::PushScope,
    symbol::Symbol,
  },
};

fn returns_or_throws(cf: ControlFlow) -> bool {
  cf == ControlFlow::Returns || cf == ControlFlow::Throws
}

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `f` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_for_in(&mut self, f: *mut AstStatForIn) -> ControlFlow {
    unsafe {
      let for_scope: *mut DfgScope = self.make_child_scope(ScopeType::Loop);

      let cf;
      {
        let _ps = PushScope::new(&mut self.scope_stack, for_scope);

        for local in (*f).vars.iter() {
          let local = *local;
          if !(*local).annotation.is_null() {
            self.visit_type_ast_type((*local).annotation);
          }

          let def =
            (*self.def_arena).fresh_cell(Symbol::from_local(local), (*local).location, false);
          *self.graph.local_defs.get_or_insert(local as *const _) = def;
          *(*self.current_scope())
            .bindings
            .get_or_insert(Symbol::from_local(local)) = def;
          self
            .captures
            .get_or_insert(Symbol::from_local(local))
            .all_versions
            .push(def);
        }

        for expr in (*f).values.iter() {
          self.visit_expr_ast_expr(*expr);
        }

        cf = self.visit_ast_stat_block((*f).body);
      }

      let scope = self.current_scope();
      if !returns_or_throws(cf) {
        self.join(scope, scope, for_scope);
      }
    }

    ControlFlow::None
  }
}
