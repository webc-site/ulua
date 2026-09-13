#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutocompleteContext {
  Unknown,
  Expression,
  Statement,
  Property,
  Type,
  Keyword,
  String,
  HotComment,
}
