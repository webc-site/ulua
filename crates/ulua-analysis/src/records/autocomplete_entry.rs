use alloc::string::String;

use crate::{
  enums::{
    autocomplete_entry_kind::AutocompleteEntryKind,
    parentheses_recommendation::ParenthesesRecommendation, type_correct_kind::TypeCorrectKind,
  },
  records::{extern_type::ExternType, property_type_path::Property},
  type_aliases::{tags::Tags, type_id::TypeId},
};

#[derive(Debug, Clone)]
pub struct AutocompleteEntry {
  pub kind: AutocompleteEntryKind,
  pub r#type: Option<TypeId>,
  pub deprecated: bool,
  pub wrong_index_type: bool,
  pub type_correct: TypeCorrectKind,
  pub containing_extern_type: Option<*const ExternType>,
  pub prop: Option<*const Property>,
  pub documentation_symbol: Option<String>,
  pub tags: Tags,
  pub parens: ParenthesesRecommendation,
  pub insert_text: Option<String>,
  pub indexed_with_self: bool,
}

impl Default for AutocompleteEntry {
  fn default() -> Self {
    Self {
      kind: AutocompleteEntryKind::Property,
      r#type: None,
      deprecated: false,
      wrong_index_type: false,
      type_correct: TypeCorrectKind::None,
      containing_extern_type: None,
      prop: None,
      documentation_symbol: None,
      tags: Tags::new(),
      parens: ParenthesesRecommendation::None,
      insert_text: None,
      indexed_with_self: false,
    }
  }
}
