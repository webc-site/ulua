use alloc::string::String;
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct RequireCompletion {
  pub label: String,
  pub insert_text: String,
}
