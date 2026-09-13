use core::mem::take;
extern crate alloc;

use alloc::{string::ToString, vec::Vec};

use crate::{
  records::{require_alias::RequireAlias, require_suggestion::RequireSuggestion},
  type_aliases::require_suggestions::RequireSuggestions,
};

pub(crate) fn make_suggestions_from_aliases(aliases: Vec<RequireAlias>) -> RequireSuggestions {
  let mut result = RequireSuggestions::with_capacity(aliases.len());
  for mut alias in aliases {
    let label = "@".to_string() + &alias.alias;
    let suggestion = RequireSuggestion {
      label: label.clone(),
      full_path: label,
      tags: take(&mut alias.tags),
    };
    result.push(suggestion);
  }
  result
}
