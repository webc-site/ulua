use ulua_ast::records::{
  ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
  ast_stat_class::AstStatClass,
};
use ulua_common::{FFlag, LUAU_ASSERT};

use crate::{
  enums::control_flow::ControlFlow,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, symbol::Symbol},
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `d` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_stat_class(&mut self, d: *mut AstStatClass) -> ControlFlow {
    LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());

    unsafe {
      let d = &*d;
      let symbol = Symbol::from_local(d.name);
      let def = (*self.def_arena).fresh_cell(symbol.clone(), (*d.name).location, false);

      *self.graph.local_defs.get_or_insert(d.name as *const _) = def;
      *(*self.current_scope())
        .bindings
        .get_or_insert(symbol.clone()) = def;
      self.captures.get_or_insert(symbol).all_versions.push(def);

      let members = &d.members;
      for member in members.as_slice() {
        if let Some(prop) = member.get_if::<AstClassProperty>() {
          if !prop.ty.is_null() {
            self.visit_type_ast_type(prop.ty);
          }
        } else if let Some(method) = member.get_if::<AstClassMethod>() {
          self.visit_expr_ast_expr_function(method.function);
        }
      }
    }

    ControlFlow::None
  }
}
