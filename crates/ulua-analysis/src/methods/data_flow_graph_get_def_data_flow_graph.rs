//! C++ `DefId DataFlowGraph::getDef(const AstExpr* expr) const`
//! (`Analysis/src/DataFlowGraph.cpp:64`). Interface-only: the body awaits the
//! full DataFlowGraph port; the signature is the contract its callers depend on.
use ulua_ast::records::{
  ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat,
  ast_stat_declare_function::AstStatDeclareFunction, ast_stat_declare_global::AstStatDeclareGlobal,
};

use crate::{records::data_flow_graph::DataFlowGraph, type_aliases::def_id_def::DefId};

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
