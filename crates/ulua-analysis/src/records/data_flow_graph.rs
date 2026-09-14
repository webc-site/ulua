//! @interface-stub
use ulua_ast::records::{ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  records::{def::Def, refinement_key::RefinementKey, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};

#[derive(Debug, Clone)]
pub struct DataFlowGraph {
  pub(crate) ast_defs: DenseHashMap<*const AstExpr, *const Def>,
  pub(crate) local_defs: DenseHashMap<*const AstLocal, *const Def>,
  pub(crate) declared_defs: DenseHashMap<*const AstStat, *const Def>,
  pub(crate) def_to_symbol: DenseHashMap<*const Def, Symbol>,
  pub(crate) ast_refinement_keys: DenseHashMap<*const AstExpr, *const RefinementKey>,
}

impl DataFlowGraph {
  /// `DefId DataFlowGraph::getDef(const AstExpr* expr) const`.
  /// Reference: `DataFlowGraph.cpp` — `getDefOptional` plus an assert.
  pub fn get_def_ast_expr(&self, expr: *const AstExpr) -> DefId {
    let def = self.ast_defs.find(&expr);
    LUAU_ASSERT!(def.is_some());
    *def.unwrap()
  }

  /// `DefId DataFlowGraph::getDef(const AstLocal* local) const`. Reference: `DataFlowGraph.cpp:79-84`.
  pub fn get_def_ast_local(&self, local: *const AstLocal) -> DefId {
    let def = self.local_defs.find(&local);
    LUAU_ASSERT!(def.is_some());
    *def.unwrap()
  }
}
