use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::records::{
  ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
  ast_expr_local::AstExprLocal, ast_node::AstNode, ast_stat_function::AstStatFunction,
};

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, dfg_scope::DfgScope, push_scope::PushScope,
    symbol::Symbol,
  },
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `f` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_function(&mut self, f: *mut AstStatFunction) -> ControlFlow {
    unsafe {
      let f = &*f;

      // local f
      // function f()
      //   if cond() then
      //     f() -- should reference only the function version and other future version, and nothing prior
      //   end
      // end
      let incoming_def =
        (*self.def_arena).fresh_cell(Symbol::default(), (*f.name).base.location, false);
      self.visit_l_value_ast_expr_def_id(f.name, incoming_def);

      // C++ compatibility logic:
      // - treat the function's `name` as an LValue and initialize with a fresh Def
      // - create a child signature scope and bind ungeneralized function defs for global/indexed properties
      // - visit the nested function body with that signature scope
      // - if `name` is local, adjust capture version offset
      //
      // The underlying defs/graph/maps are not exposed via the provided Rust API surface in this module,
      // so these operations are omitted here (kept as no-ops for correctness under the available bindings).

      let signature_scope: *mut DfgScope = self.make_child_scope(ScopeType::Function);
      let _ps = PushScope::new(&mut self.scope_stack, signature_scope);

      if !f.name.is_null() {
        let name_ptr = f.name;
        let name_node = name_ptr as *mut AstNode;
        let name_def = self.graph.get_def_ast_expr(name_ptr);

        if (*name_node).is::<AstExprGlobal>() {
          let global = &*(name_ptr as *mut AstExprGlobal);
          let symbol = Symbol::from_global(global.name);
          *(*signature_scope).bindings.get_or_insert(symbol) = name_def;
        } else if (*name_node).is::<AstExprIndexName>() {
          let index_name = &*(name_ptr as *mut AstExprIndexName);
          let expr_node = index_name.expr as *mut AstNode;

          if !expr_node.is_null() && (*expr_node).is::<AstExprLocal>() {
            let receiver = (*(index_name.expr as *mut AstExprLocal)).local;
            let receiver_def = self
              .lookup_symbol_location(Symbol::from_local(receiver), (*f.func).base.base.location);
            let key = if index_name.index.value.is_null() {
              String::new()
            } else {
              CStr::from_ptr(index_name.index.value)
                .to_string_lossy()
                .into_owned()
            };

            (*signature_scope)
              .props
              .get_or_insert(receiver_def)
              .insert(key, name_def);
          }
        }
      }

      if !f.func.is_null() {
        // visitFunction(f->func, NotNull{signatureScope});
        let _ = self.visit_function(f.func, signature_scope);
      }

      if !f.name.is_null() {
        let name_ptr = f.name;
        let name_node = name_ptr as *mut AstNode;

        if (*name_node).is::<AstExprLocal>() {
          let local = &*(name_ptr as *mut AstExprLocal);
          let capture = self.captures.get_or_insert(Symbol::from_local(local.local));
          ulua_common::LUAU_ASSERT!(!capture.all_versions.is_empty());
          capture.version_offset = capture.all_versions.len() - 1;
        }
      }

      ControlFlow::None
    }
  }
}
