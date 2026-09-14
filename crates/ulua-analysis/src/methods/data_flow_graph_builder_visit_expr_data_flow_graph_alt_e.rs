use core::{ffi::c_void, ptr::null};

use ulua_ast::{
  functions::is_l_value::is_l_value,
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
};
use ulua_common::{
  FFlag,
  macros::{luau_assert::LUAU_ASSERT, luau_unreachable::LUAU_UNREACHABLE},
};

use crate::{
  enums::scope_type::ScopeType,
  functions::should_typestate_for_first_argument::should_typestate_for_first_argument,
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, def::Def,
    dfg_scope::DfgScope, symbol::Symbol,
  },
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `c` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_call(&mut self, c: *mut AstExprCall) -> DataFlowResult {
    unsafe {
      let call = &*c;

      self.visit_expr_ast_expr(call.func);

      if FFlag::LuauVisitCallTypeArgsInDfg.get()
        && FFlag::LuauExplicitTypeInstantiationSupport.get()
      {
        for type_or_pack in call.type_arguments.iter() {
          if !type_or_pack.r#type.is_null() {
            self.visit_type_ast_type(type_or_pack.r#type);
          } else {
            LUAU_ASSERT!(!type_or_pack.type_pack.is_null());
            self.visit_type_pack_ast_type_pack(type_or_pack.type_pack);
          }
        }
      }

      for arg in call.args.iter() {
        self.visit_expr_ast_expr(*arg);
      }

      if should_typestate_for_first_argument(call)
        && call.args.size > 1
        && is_l_value(*call.args.data as *const AstExpr)
      {
        let first_arg = *call.args.data;
        let first_node = first_arg as *mut AstNode;

        let result = if (*first_node).is::<AstExprLocal>() {
          self.visit_expr_ast_expr_local(first_arg as *mut AstExprLocal)
        } else if (*first_node).is::<AstExprGlobal>() {
          self.visit_expr_ast_expr_global(first_arg as *mut AstExprGlobal)
        } else if (*first_node).is::<AstExprIndexName>() {
          self.visit_expr_ast_expr_index_name(first_arg as *mut AstExprIndexName)
        } else if (*first_node).is::<AstExprIndexExpr>() {
          self.visit_expr_ast_expr_index_expr(first_arg as *mut AstExprIndexExpr)
        } else {
          LUAU_UNREACHABLE!();
        };

        let child: *mut DfgScope = self.make_child_scope(ScopeType::Linear);
        self.scope_stack.push(child);

        *self
          .graph
          .ast_defs
          .get_or_insert(first_arg as *const AstExpr) = result.def as *const Def;
        if !result.parent.is_null() {
          *self
            .graph
            .ast_refinement_keys
            .get_or_insert(first_arg as *const AstExpr) = result.parent;
        }

        self.visit_l_value_ast_expr_def_id(first_arg, result.def as *const Def);
      }

      let fresh_def =
        (*self.def_arena).fresh_cell(Symbol::default(), call.base.base.location, true);
      DataFlowResult {
        def: fresh_def as *const c_void,
        parent: null(),
      }
    }
  }
}
