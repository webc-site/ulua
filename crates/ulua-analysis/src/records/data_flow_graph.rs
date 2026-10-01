use ulua_ast::records::{ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  records::{refinement_key::RefinementKey, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};

#[derive(Debug, Clone)]
pub struct DataFlowGraph {
  pub(crate) ast_defs: DenseHashMap<*const AstExpr, DefId>,
  pub(crate) local_defs: DenseHashMap<*const AstLocal, DefId>,
  pub(crate) declared_defs: DenseHashMap<*const AstStat, DefId>,
  pub(crate) def_to_symbol: DenseHashMap<DefId, Symbol>,
  pub(crate) ast_refinement_keys: DenseHashMap<*const AstExpr, *const RefinementKey>,
}

impl DataFlowGraph {
  /// `DefId DataFlowGraph::getDef(const AstExpr* expr) const`.
  /// Reference: `DataFlowGraph.cpp` — `getDefOptional` plus an assert.
  pub fn get_def_ast_expr(&self, expr: *const AstExpr) -> DefId {
    let def = self.ast_defs.find(&expr);
    LUAU_ASSERT!(def.is_some());
    // 紧邻 LUAU_ASSERT 蕴含 Some（cpp `getDef` 的 assert+解引用同前提）。
    *def.expect("紧邻 LUAU_ASSERT(def.is_some()) 蕴含")
  }

  /// `DefId DataFlowGraph::getDef(const AstLocal* local) const`. Reference: `DataFlowGraph.cpp:79-84`.
  pub fn get_def_ast_local(&self, local: *const AstLocal) -> DefId {
    let def = self.local_defs.find(&local);
    LUAU_ASSERT!(def.is_some());
    // 紧邻 LUAU_ASSERT 蕴含 Some（cpp `getDef` 的 assert+解引用同前提）。
    *def.expect("紧邻 LUAU_ASSERT(def.is_some()) 蕴含")
  }
}
