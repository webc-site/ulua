use core::ptr::null;

use ulua_ast::records::ast_expr_function::AstExprFunction;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::arena_ref::arena_ref,
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult,
    dfg_scope::DfgScope, symbol::Symbol,
  },
  type_aliases::def_id_def::DefId,
};

/// 把 `def` 写进签名作用域的 `bindings`（cpp `(*signatureScope)[symbol] = def`）。
///
/// # Safety
/// `signature_scope` 须为非空且指向 PinnedStorage 中地址稳定的活 `DfgScope`。
#[inline]
unsafe fn bind_in_signature_scope(signature_scope: *mut DfgScope, symbol: Symbol, def: DefId) {
  // SAFETY: 由本函数 Safety 契约保证 scope 非空存活；仅写其 bindings 存储，
  // 与 localDefs/def 表不重叠。
  unsafe { *(*signature_scope).bindings.get_or_insert(symbol) = def };
}

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `f` 非空且指向 parse arena 存活节点、`signature_scope` 非空
  /// 且指向 PinnedStorage 中地址稳定的活 `DfgScope`，满足 C++ 原实现的调用契约。
  pub unsafe fn visit_function(
    &mut self,
    f: *mut AstExprFunction,
    signature_scope: *mut DfgScope,
  ) -> DataFlowResult {
    // SAFETY: `f` 由本函数 Safety 契约保证为非空 arena 存活节点，分析期只读。
    let f_ref: &AstExprFunction = arena_ref(f, "visit_function.f");

    // SAFETY: self_ 是显式可空字段，as_ref 把判空折叠进取引用（cpp
    // `if (f->self)` 同款）；命中即 arena 存活只读 AstLocal 节点。
    if let Some(self_local) = f_ref.self_.as_ref() {
      // There's no syntax for `self` to have an annotation if using `function t:m()`
      LUAU_ASSERT!(self_local.annotation.is_null());

      let self_local_ptr = f_ref.self_.as_ptr();
      let symbol = Symbol::from_local(self_local_ptr);
      let def = self.def_arena.get_mut().fresh_cell(
        Symbol::from_global(f_ref.debugname),
        f_ref.base.base.location,
        false,
      );
      *self
        .graph
        .local_defs
        .get_or_insert(self_local_ptr as *const _) = def;
      // SAFETY: signature_scope 由本函数 Safety 契约保证满足被调前置条件。
      unsafe { bind_in_signature_scope(signature_scope, symbol.clone(), def) };
      self.captures.get_or_insert(symbol).all_versions.push(def);
    }

    for param_node in f_ref.args.iter_nodes() {
      let param = param_node.get();
      let param_ptr = param_node.as_ptr();
      if let Some(annotation) = unsafe { param.annotation.as_ref() } {
        self.visit_type(annotation);
      }

      let symbol = Symbol::from_local(param_ptr);
      let def = self
        .def_arena
        .get_mut()
        .fresh_cell(symbol.clone(), param.location, false);
      *self.graph.local_defs.get_or_insert(param_ptr as *const _) = def;
      // SAFETY: 同上——signature_scope 契约由本函数 Safety 前提保证。
      unsafe { bind_in_signature_scope(signature_scope, symbol.clone(), def) };
      self.captures.get_or_insert(symbol).all_versions.push(def);
    }

    if let Some(vararg_annotation) = f_ref.vararg_annotation.as_ref() {
      self.visit_type_pack(vararg_annotation);
    }

    if let Some(return_annotation) = f_ref.return_annotation.as_ref() {
      self.visit_type_pack(return_annotation);
    }

    let body = f_ref.body.get();
    self.visit_stat_block(body);

    DataFlowResult {
      def: self.def_arena.get_mut().fresh_cell(
        Symbol::from_global(f_ref.debugname),
        f_ref.base.base.location,
        false,
      ),
      parent: null(),
    }
  }
}
