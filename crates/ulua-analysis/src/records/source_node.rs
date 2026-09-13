use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::type_aliases::module_name_type::ModuleName;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceNode {
  pub name: ModuleName,
  pub human_readable_name: String,
  pub require_set: DenseHashSet<ModuleName>,
  pub require_locations: Vec<(ModuleName, Location)>,
  pub dependents: DenseHashSet<ModuleName>,
  pub dirty_source_module: bool,
  pub dirty_module: bool,
  pub dirty_module_for_autocomplete: bool,
  pub invalid_module_dependency: bool,
  pub invalid_module_dependency_for_autocomplete: bool,
  pub autocomplete_limits_mult: f64,
}

impl SourceNode {
  pub fn has_dirty_source_module(&self) -> bool {
    self.dirty_source_module
  }

  pub fn has_dirty_module(&self, for_autocomplete: bool) -> bool {
    if for_autocomplete {
      self.dirty_module_for_autocomplete
    } else {
      self.dirty_module
    }
  }

  pub fn has_invalid_module_dependency(&self, for_autocomplete: bool) -> bool {
    if for_autocomplete {
      self.invalid_module_dependency_for_autocomplete
    } else {
      self.invalid_module_dependency
    }
  }

  pub fn set_invalid_module_dependency(&mut self, value: bool, for_autocomplete: bool) {
    if for_autocomplete {
      self.invalid_module_dependency_for_autocomplete = value;
    } else {
      self.invalid_module_dependency = value;
    }
  }
}
