use alloc::{string::ToString, vec::Vec};

use crate::{
  records::{data_flow_graph::DataFlowGraph, usage_finder::UsageFinder},
  type_aliases::name_type::Name,
};

impl UsageFinder {
  pub fn new(dfg: *mut DataFlowGraph) -> Self {
    // Field initializers: referencedBindings{""}, referencedImportedBindings{{"", ""}}.
    // We explicitly suggest that the usage finder populate types for instance and enum by default
    // These are common enough types that sticking them in the environment is a good idea
    // and it lets magic functions work correctly too.
    let referenced_bindings: Vec<Name> =
      alloc::vec!["".to_string(), "Instance".to_string(), "Enum".to_string()];
    let referenced_imported_bindings: Vec<(Name, Name)> =
      alloc::vec![("".to_string(), "".to_string())];

    UsageFinder {
      dfg,
      declared_aliases: Default::default(),
      local_bindings_referenced: Vec::new(),
      mentioned_defs: Default::default(),
      referenced_bindings,
      referenced_imported_bindings,
      global_defs_to_pre_populate: Vec::new(),
      global_functions_referenced: Vec::new(),
      symbols_to_refine: Vec::new(),
    }
  }
}
