use alloc::collections::BTreeMap;

use crate::{
  enums::autocomplete_entry_kind::AutocompleteEntryKind,
  records::autocomplete_entry::AutocompleteEntry,
  type_aliases::{
    autocomplete_entry_map::AutocompleteEntryMap, require_suggestions::RequireSuggestions,
  },
};

pub fn convert_require_suggestions_to_autocomplete_entry_map(
  suggestions: Option<RequireSuggestions>,
) -> Option<AutocompleteEntryMap> {
  let suggestions = suggestions?;

  let mut result = BTreeMap::new();
  for suggestion in suggestions {
    let entry = AutocompleteEntry {
      kind: AutocompleteEntryKind::RequirePath,
      insert_text: Some(suggestion.full_path),
      tags: suggestion.tags,
      ..Default::default()
    };

    result.insert(suggestion.label, entry);
  }

  Some(result)
}
