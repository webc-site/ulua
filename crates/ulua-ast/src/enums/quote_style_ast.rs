#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuoteStyle {
  QuotedSimple,
  QuotedSingle,
  QuotedRaw,
  Unquoted,
}
