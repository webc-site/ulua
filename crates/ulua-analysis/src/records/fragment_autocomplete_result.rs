use crate::{
  records::{autocomplete_result::AutocompleteResult, scope::Scope},
  type_aliases::module_ptr_module::ModulePtr,
};

#[derive(Debug, Clone)]
pub struct FragmentAutocompleteResult {
  pub incremental_module: ModulePtr,
  pub fresh_scope: *mut Scope,
  pub ac_results: AutocompleteResult,
}
