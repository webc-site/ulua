use crate::{
  enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
  records::fragment_autocomplete_result::FragmentAutocompleteResult,
};

#[derive(Debug, Clone)]
pub struct FragmentAutocompleteStatusResult {
  pub status: FragmentAutocompleteStatus,
  pub result: Option<FragmentAutocompleteResult>,
}
