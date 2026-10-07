use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  enums::scope_type::ScopeType,
  records::{
    arena_handle::{alias, alias_ref},
    cell::Cell,
    data_flow_graph_builder::DataFlowGraphBuilder,
    def_registry::{def_as, def_ref},
    dfg_scope::DfgScope,
    phi::Phi,
    symbol::Symbol,
  },
  type_aliases::def_id_def::DefId,
};

/// 存活句柄的符号名（cpp `def->getName()`）；空/异线程句柄属契约违例。
fn def_name(def: DefId) -> Symbol {
  def_ref(def).expect("Def arena 存活句柄契约").name.clone()
}

impl DataFlowGraphBuilder {
  pub fn lookup_symbol_location(&mut self, symbol: Symbol, location: Location) -> DefId {
    let scope = self.current_scope();

    let mut outside_loop_scope = false;
    let mut current: *mut DfgScope = scope;
    while !current.is_null() {
      outside_loop_scope = outside_loop_scope || alias_ref(current).scope_type == ScopeType::Loop;

      if let Some(found) = alias_ref(current).bindings.find(&symbol) {
        return *found;
      } else if alias_ref(current).scope_type == ScopeType::Function {
        let capture = self.captures.get_or_insert(symbol.clone());
        let capture_def = self.def_arena.get_mut().phi_vector_def_id(&Vec::new());
        capture.capture_defs.push(capture_def);

        if !outside_loop_scope {
          *alias(scope).bindings.get_or_insert(symbol.clone()) = capture_def;
        }

        return capture_def;
      }

      current = alias_ref(current).parent;
    }

    let result = self
      .def_arena
      .get_mut()
      .fresh_cell(symbol.clone(), location, false);
    *alias(scope).bindings.get_or_insert(symbol.clone()) = result;
    self
      .captures
      .get_or_insert(symbol)
      .all_versions
      .push(result);
    result
  }

  /// `DefId DataFlowGraphBuilder::lookup(DefId def, const std::string& key, Location location)`.
  /// Reference: `DataFlowGraph.cpp:328-364`.
  /// # Safety
  pub(crate) fn lookup_def_id_string_location(
    &mut self,
    def: DefId,
    key: &str,
    location: Location,
  ) -> DefId {
    let scope = self.current_scope();

    let mut current: *mut DfgScope = scope;
    while !current.is_null() {
      if let Some(props) = alias_ref(current).props.find(&def) {
        if let Some(found) = props.get(key) {
          return *found;
        }
      } else if let Some(phi) = def_as::<Phi>(def)
        && phi.operands.is_empty()
        && alias_ref(current).scope_type == ScopeType::Function
      {
        let result = self
          .def_arena
          .get_mut()
          .fresh_cell(def_name(def), location, false);
        alias(scope)
          .props
          .get_or_insert(def)
          .insert(key.to_owned(), result);
        return result;
      }

      current = alias_ref(current).parent;
    }

    // `def` 是注册表句柄，节点探测经安全的 `def_as` 下转；`self.def_arena`、
    // `scope` 构造接线非空、块地址稳定；递归传入的 operand 仍是存活合法句柄；
    // 兜底分支的 `self.handle` 由 build() 接线非空（C++ `NotNull` 形参契约）。
    if let Some(phi) = def_as::<Phi>(def) {
      let mut defs = Vec::new();
      for operand in &phi.operands {
        defs.push(self.lookup_def_id_string_location(*operand, key, location));
      }

      let result = self.def_arena.get_mut().phi_vector_def_id(&defs);
      alias(scope)
        .props
        .get_or_insert(def)
        .insert(key.to_owned(), result);
      return result;
    }

    if def_as::<Cell>(def).is_some() {
      let result = self
        .def_arena
        .get_mut()
        .fresh_cell(def_name(def), location, false);
      alias(scope)
        .props
        .get_or_insert(def)
        .insert(key.to_owned(), result);
      return result;
    }

    self
      .handle
      .expect("DataFlowGraphBuilder::handle 须由 build() 接线非空")
      .get()
      .ice_string("Inexhaustive lookup cases in DataFlowGraphBuilder::lookup");
    unreachable!()
  }
}
