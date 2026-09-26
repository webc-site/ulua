use ulua_ast::{
  functions::optional_node::node_ref,
  records::{ast_array::AstArray, ast_generic_type_pack::AstGenericTypePack},
};

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  /// cpp `visitGenericPacks`：登记泛型型包形参的默认类型包（若有）。
  pub fn visit_generic_packs(&mut self, g: AstArray<*mut AstGenericTypePack>) {
    for &generic in g.as_slice() {
      // SAFETY: 数组元素可为 null（原实现的显式跳空守卫保留），as_ref 把
      // 判空折叠进取引用；命中即 arena 存活只读节点。
      if let Some(node) = unsafe { generic.as_ref() }
        && let Some(default_value) = node_ref(node.default_value)
      {
        self.visit_type_pack(default_value);
      }
    }
  }
}
