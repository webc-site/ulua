//! `data_flow_graph` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::null;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat,
  ast_stat_declare_function::AstStatDeclareFunction, ast_stat_declare_global::AstStatDeclareGlobal,
};

use crate::{
  records::{data_flow_graph::DataFlowGraph, refinement_key::RefinementKey, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};

// C++ `DefId DataFlowGraph::getDef(const AstExpr* expr) const`
// (`Analysis/src/DataFlowGraph.cpp:64`). Interface-only: the body awaits the
// full DataFlowGraph port; the signature is the contract its callers depend on.

impl DataFlowGraph {
  pub fn get_def(&self, expr: *const AstExpr) -> DefId {
    // C++: auto def = astDefs.find(expr); LUAU_ASSERT(def); return NotNull{*def};
    let def = self.ast_defs.find(&expr);
    *def.expect("cpp LUAU_ASSERT(def)：可见性分析已为每个存活 expr 建 def")
  }
}
// C++ `DefId DataFlowGraph::getDef(const AstLocal* local) const`
// (`Analysis/src/DataFlowGraph.cpp:79`). Looks the def up in `localDefs` by the
// `AstLocal*` key and asserts it is present.
impl DataFlowGraph {
  pub fn get_def_for_local(&self, local: *const AstLocal) -> DefId {
    // C++: auto def = localDefs.find(local); LUAU_ASSERT(def); return NotNull{*def};
    let def = self.local_defs.find(&local);
    *def.expect("cpp LUAU_ASSERT(def)：每个 AstLocal 在建图期登记 def")
  }
}
// C++ `DefId DataFlowGraph::getDef(const AstStatDeclareFunction* func) const`
// (`Analysis/src/DataFlowGraph.cpp:93`). Looks the def up in `declaredDefs`,
// whose keys are `AstStat*`, so the `AstStatDeclareFunction*` is upcast first.
impl DataFlowGraph {
  pub fn get_def_for_declare_function(&self, func: *const AstStatDeclareFunction) -> DefId {
    // C++: auto def = declaredDefs.find(func); LUAU_ASSERT(def); return NotNull{*def};
    let def = self.declared_defs.find(&(func as *const AstStat));
    *def.expect("cpp LUAU_ASSERT(def)：declare function 建图期登记 def")
  }
}
// C++ `DefId DataFlowGraph::getDef(const AstStatDeclareGlobal* global) const`
// (`Analysis/src/DataFlowGraph.cpp:86`).
impl DataFlowGraph {
  pub fn get_def_declare_global(&self, global: *const AstStatDeclareGlobal) -> DefId {
    let def = self.declared_defs.find(&(global as *const AstStat));
    *def.expect("cpp LUAU_ASSERT(def)：declare global 建图期登记 def")
  }
}
// C++ `DefId DataFlowGraph::getDef(const AstLocal* local) const`
// (`Analysis/src/DataFlowGraph.cpp:79`).
impl DataFlowGraph {
  pub fn get_def_local(&self, local: *const AstLocal) -> DefId {
    let def = self.local_defs.find(&local);
    *def.expect("cpp LUAU_ASSERT(def)：每个 AstLocal 在建图期登记 def")
  }
}

impl DataFlowGraph {
  pub fn get_def_optional(&self, expr: *const AstExpr) -> Option<DefId> {
    // C++: auto def = astDefs.find(expr); if (!def) return nullopt; return NotNull{*def};
    self.ast_defs.find(&expr).copied()
  }
}

impl DataFlowGraph {
  pub fn get_refinement_key(&self, expr: *const AstExpr) -> *const RefinementKey {
    if let Some(v) = self.ast_refinement_keys.find(&expr) {
      *v
    } else {
      null()
    }
  }
}

impl DataFlowGraph {
  pub fn get_symbol_from_def(&self, def: DefId) -> Option<Symbol> {
    // C++: if (auto ref = defToSymbol.find(def)) return *ref; return nullopt;
    self.def_to_symbol.find(&def).cloned()
  }
}
