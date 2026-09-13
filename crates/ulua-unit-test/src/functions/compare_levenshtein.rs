use core::str::from_utf8_unchecked;

use ulua_common::functions::edit_distance::editDistance;

use crate::{functions::format::format, type_aliases::levenshtein_matrix::LevenshteinMatrix};
pub fn compare_levenshtein(distances: LevenshteinMatrix, a: &str, b: &str) {
  let a_bytes = a.as_bytes();
  let b_bytes = b.as_bytes();

  for x in 0..=a_bytes.len() {
    for y in 0..=b_bytes.len() {
      let current_a = &a_bytes[..x];
      let current_b = &b_bytes[..y];

      let actual = editDistance(current_a, current_b);
      let expected = distances[x][y];

      if actual != expected {
        let message = format(
          unsafe { from_utf8_unchecked(current_a) },
          unsafe { from_utf8_unchecked(current_b) },
          expected,
          actual,
        );
        panic!("{}", message);
      }
    }
  }
}
