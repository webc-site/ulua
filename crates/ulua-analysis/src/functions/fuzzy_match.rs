use ulua_common::{FInt::LuauSuggestionDistance, functions::edit_distance::editDistance};

pub fn fuzzy_match<'a>(str: &str, candidates: &[&'a str]) -> Option<&'a str> {
  let suggestion_distance = LuauSuggestionDistance.get() as usize;
  if suggestion_distance == 0 {
    return None;
  }

  let bytes = str.as_bytes();

  let mut best_distance = suggestion_distance;
  let mut best_match = candidates.len();

  for (i, candidate) in candidates.iter().enumerate() {
    let ed = editDistance(bytes, candidate.as_bytes());
    if ed <= best_distance {
      best_distance = ed;
      best_match = i;
    }
  }

  if best_match < candidates.len() {
    Some(candidates[best_match])
  } else {
    None
  }
}
