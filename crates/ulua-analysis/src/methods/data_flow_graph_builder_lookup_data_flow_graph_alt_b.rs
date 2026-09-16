use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;

use crate::{
  enums::scope_type::ScopeType,
  functions::get_def::get_def_id,
  records::{
    cell::Cell, data_flow_graph_builder::DataFlowGraphBuilder, dfg_scope::DfgScope, phi::Phi,
  },
  type_aliases::def_id_def::DefId,
};
impl DataFlowGraphBuilder {
  /// `DefId DataFlowGraphBuilder::lookup(DefId def, const std::string& key, Location location)`.
  /// Reference: `DataFlowGraph.cpp:328-364`.
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn lookup_def_id_string_location(
    &mut self,
    def: DefId,
    key: &String,
    location: Location,
  ) -> DefId {
    let scope = self.current_scope();

    let mut current: *mut DfgScope = scope;
    while !current.is_null() {
      unsafe {
        if let Some(props) = (*current).props.find(&def) {
          if let Some(found) = props.get(key) {
            return *found;
          }
        } else {
          let phi = get_def_id::<Phi>(def);
          if !phi.is_null()
            && (*phi).operands.is_empty()
            && (*current).scope_type == ScopeType::Function
          {
            let result = (*self.def_arena).fresh_cell((*def).name.clone(), location, false);
            (*scope)
              .props
              .get_or_insert(def)
              .insert(key.clone(), result);
            return result;
          }
        }

        current = (*current).parent;
      }
    }

    unsafe {
      let phi = get_def_id::<Phi>(def);
      if !phi.is_null() {
        let mut defs = Vec::new();
        for operand in &(*phi).operands {
          defs.push(self.lookup_def_id_string_location(*operand, key, location));
        }

        let result = (*self.def_arena).phi_vector_def_id(&defs);
        (*scope)
          .props
          .get_or_insert(def)
          .insert(key.clone(), result);
        result
      } else if !get_def_id::<Cell>(def).is_null() {
        let result = (*self.def_arena).fresh_cell((*def).name.clone(), location, false);
        (*scope)
          .props
          .get_or_insert(def)
          .insert(key.clone(), result);
        result
      } else {
        (*self.handle).ice_string("Inexhaustive lookup cases in DataFlowGraphBuilder::lookup");
        unreachable!()
      }
    }
  }
}
