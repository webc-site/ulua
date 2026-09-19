#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutocompleteEntryKind {
  Property,
  Binding,
  Keyword,
  String,
  Type,
  Module,
  GeneratedFunction,
  RequirePath,
  HotComment,
}
