use ulua_ast::records::{ast_type::AstType, ast_type_list::AstTypeList};

use crate::{
  functions::arena_ref::arena_ref, records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  /// cpp `visitTypeList`：逐类型登记，尾型包非空时继续分派。
  pub fn visit_type_list(&mut self, l: AstTypeList) {
    for &t in l.types.as_slice() {
      // SAFETY: 数组元素由 parser 契约保证非空（cpp 直接解引用）。
      let t = arena_ref::<AstType>(t, "AstTypeList.types 元素");
      self.visit_type(t);
    }

    // 尾型包可空：cpp `if (l.tailType)` 同款，判空折叠进取引用。
    // SAFETY: 命中即 arena 存活只读节点。
    if let Some(tail_type) = unsafe { l.tail_type.as_ref() } {
      self.visit_type_pack(tail_type);
    }
  }
}
