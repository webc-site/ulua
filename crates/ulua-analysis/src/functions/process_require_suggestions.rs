use ulua_common::functions::escape::escape;

use crate::type_aliases::require_suggestions::RequireSuggestions;

pub fn process_require_suggestions(
  mut suggestions: Option<RequireSuggestions>,
) -> Option<RequireSuggestions> {
  if let Some(ref mut suggestions_vec) = suggestions {
    for suggestion in suggestions_vec {
      suggestion.full_path = escape(&suggestion.full_path, false);
    }
  }

  suggestions
}
