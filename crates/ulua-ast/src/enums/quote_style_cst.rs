#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuoteStyle {
  QuotedSingle,
  QuotedDouble,
  QuotedRaw,
  QuotedInterp,
}
