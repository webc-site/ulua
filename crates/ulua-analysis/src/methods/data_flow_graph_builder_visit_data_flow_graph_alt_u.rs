use ulua_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  records::{data_flow_graph_builder::DataFlowGraphBuilder, push_scope::PushScope},
};
impl DataFlowGraphBuilder {
  pub(crate) fn visit_ast_stat_declare_extern_type(
    &mut self,
    d: *mut AstStatDeclareExternType,
  ) -> ControlFlow {
    unsafe {
      let d = &*d;

      // This declaration does not "introduce" any bindings in value namespace,
      // so there's no symbolic value to begin with. We'll traverse the properties
      // because their type annotations may depend on something in the value namespace.
      let unreachable = self.make_child_scope(ScopeType::Linear);
      let mut _ps = PushScope::new(&mut self.scope_stack, unreachable);

      for &prop in d.props.as_slice() {
        self.visit_type_ast_type(prop.ty);
      }
    }

    ControlFlow::None
  }
}
