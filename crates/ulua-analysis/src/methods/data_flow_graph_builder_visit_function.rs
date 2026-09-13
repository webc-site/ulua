use core::{ffi::c_void, ptr::null};

use ulua_ast::records::ast_expr_function::AstExprFunction;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult,
  dfg_scope::DfgScope, symbol::Symbol,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_function(
    &mut self,
    f: *mut AstExprFunction,
    signature_scope: *mut DfgScope,
  ) -> DataFlowResult {
    unsafe {
      let f_ref = &*f;

      let self_local = f_ref.self_;
      if !self_local.is_null() {
        let self_local_ref = &*self_local;
        // There's no syntax for `self` to have an annotation if using `function t:m()`
        LUAU_ASSERT!(self_local_ref.annotation.is_null());

        let symbol = Symbol::from_local(self_local);
        let def = (*self.def_arena).fresh_cell(
          Symbol::from_global(f_ref.debugname),
          f_ref.base.base.location,
          false,
        );
        *self.graph.local_defs.get_or_insert(self_local as *const _) = def;
        *(*signature_scope).bindings.get_or_insert(symbol.clone()) = def;
        self.captures.get_or_insert(symbol).all_versions.push(def);
      }

      for &param_ptr in f_ref.args.as_slice() {
        if param_ptr.is_null() {
          continue;
        }

        let param = &*param_ptr;
        if !param.annotation.is_null() {
          self.visit_type_ast_type(param.annotation);
        }

        let symbol = Symbol::from_local(param_ptr);
        let def = (*self.def_arena).fresh_cell(symbol.clone(), param.location, false);
        *self.graph.local_defs.get_or_insert(param_ptr as *const _) = def;
        *(*signature_scope).bindings.get_or_insert(symbol.clone()) = def;
        self.captures.get_or_insert(symbol).all_versions.push(def);
      }

      if !f_ref.vararg_annotation.is_null() {
        self.visit_type_pack_ast_type_pack(f_ref.vararg_annotation);
      }

      if !f_ref.return_annotation.is_null() {
        self.visit_type_pack_ast_type_pack(f_ref.return_annotation);
      }

      self.visit_ast_stat_block(f_ref.body);

      DataFlowResult {
        def: (*self.def_arena).fresh_cell(
          Symbol::from_global(f_ref.debugname),
          f_ref.base.base.location,
          false,
        ) as *const c_void,
        parent: null(),
      }
    }
  }
}
