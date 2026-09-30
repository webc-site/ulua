extern crate alloc;

pub(crate) mod enums;
pub mod functions;
pub(crate) mod methods;
pub mod records;
pub mod type_aliases;

pub use functions::{
  generate_spans::generate_spans, main::run as run_main, pruned_span::pruned_span,
};
pub use records::{
  node::{Block, Node, Stat},
  reducer::Reducer,
};
pub use type_aliases::span::Span;
