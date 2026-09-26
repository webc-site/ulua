use alloc::{string::String, vec::Vec};
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RequireSuggestion {
  pub label: String,
  pub full_path: String,
  pub tags: Vec<String>,
}
