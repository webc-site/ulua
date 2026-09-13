mod _inner {
  pub fn starts_with(haystack: &str, needle: &str) -> bool {
    haystack.starts_with(needle)
  }
}

pub use _inner::{starts_with, starts_with as startsWith};
