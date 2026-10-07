use core::ptr::null;

use ulua_ast::records::ast_expr_function::AstExprFunction;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    arena_handle::{alias, alias_opt},
    data_flow_graph_builder::DataFlowGraphBuilder,
    data_flow_result::DataFlowResult,
    dfg_scope::DfgScope,
    symbol::Symbol,
  },
  type_aliases::def_id_def::DefId,
};

/// 把 `def` 写进签名作用域的 `bindings`（cpp `(*signatureScope)[symbol] = def`）。
///
/// 契约：`signature_scope` 为 `make_child_scope` 划出的 PinnedStorage 活单元
/// （非空、地址稳定），解引用收口在 `arena_handle::alias` 门面。
#[inline]
fn bind_in_signature_scope(signature_scope: *mut DfgScope, symbol: Symbol, def: DefId) {
  *alias(signature_scope).bindings.get_or_insert(symbol) = def;
}

impl DataFlowGraphBuilder {
  /// cpp `visitFunction(AstExprFunction* f, NotNull<DfgScope*> signatureScope)`。
  ///
  /// `f` 直接以共享借用进入（调用方持有 parse arena 存活节点，分析期只读）；
  /// `signature_scope` 为 `make_child_scope` 划出的 PinnedStorage 活单元
  /// （非空、地址稳定，存活至 builder 析构），经 `alias` 门面物化借用。
  pub(crate) fn visit_function(
    &mut self,
    f: &AstExprFunction,
    signature_scope: *mut DfgScope,
  ) -> DataFlowResult {
    // self_ 是显式可空字段，as_ref 把判空折叠进取引用（cpp `if (f->self)`
    // 同款）；命中即 arena 存活只读 AstLocal 节点。
    if let Some(self_local) = f.self_.as_ref() {
      // There's no syntax for `self` to have an annotation if using `function t:m()`
      LUAU_ASSERT!(self_local.annotation.is_null());

      let self_local_ptr = f.self_.as_ptr();
      let symbol = Symbol::from_local(self_local_ptr);
      let def = self.def_arena.get_mut().fresh_cell(
        Symbol::from_global(f.debugname),
        f.base.base.location,
        false,
      );
      *self.graph.local_defs.get_or_insert(self_local_ptr) = def;
      bind_in_signature_scope(signature_scope, symbol.clone(), def);
      self.captures.get_or_insert(symbol).all_versions.push(def);
    }

    for param_node in f.args.iter_nodes() {
      let param = param_node.get();
      let param_ptr = param_node.as_ptr();
      if let Some(annotation) = alias_opt(param.annotation) {
        self.visit_type(annotation);
      }

      let symbol = Symbol::from_local(param_ptr);
      let def = self
        .def_arena
        .get_mut()
        .fresh_cell(symbol.clone(), param.location, false);
      *self.graph.local_defs.get_or_insert(param_ptr) = def;
      bind_in_signature_scope(signature_scope, symbol.clone(), def);
      self.captures.get_or_insert(symbol).all_versions.push(def);
    }

    if let Some(vararg_annotation) = f.vararg_annotation.as_ref() {
      self.visit_type_pack(vararg_annotation);
    }

    if let Some(return_annotation) = f.return_annotation.as_ref() {
      self.visit_type_pack(return_annotation);
    }

    let body = f.body.get();
    self.visit_stat_block(body);

    DataFlowResult {
      def: self.def_arena.get_mut().fresh_cell(
        Symbol::from_global(f.debugname),
        f.base.base.location,
        false,
      ),
      parent: null(),
    }
  }
}
