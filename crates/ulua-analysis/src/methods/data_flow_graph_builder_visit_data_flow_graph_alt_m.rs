use alloc::vec::Vec;

use ulua_ast::records::ast_stat_assign::AstStatAssign;

use crate::{
  enums::control_flow::ControlFlow,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, def::Def, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `a` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_assign(&mut self, a: *mut AstStatAssign) -> ControlFlow {
    unsafe {
      let mut defs: Vec<DefId> = Vec::with_capacity((*a).values.size);

      for expr in (*a).values.iter() {
        defs.push(self.visit_expr_ast_expr(*expr).def as *const Def);
      }

      for (i, var) in (*a).vars.iter().enumerate() {
        let var = *var;
        let incoming_def = if i < defs.len() {
          defs[i]
        } else {
          (*self.def_arena).fresh_cell(Symbol::default(), (*var).base.location, false)
        };
        self.visit_l_value_ast_expr_def_id(var, incoming_def);
      }
    }

    ControlFlow::None
  }
}
