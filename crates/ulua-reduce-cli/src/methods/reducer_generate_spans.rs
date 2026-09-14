use alloc::vec::Vec;
use core::cmp::{max, min};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::reducer::Reducer, type_aliases::span::Span};

impl Reducer {
  pub fn generate_spans(&self, size: usize, chunks: usize) -> Vec<(Span, Span)> {
    if size <= 1 {
      return Vec::new();
    }

    LUAU_ASSERT!(chunks > 0);
    let chunk_length = max(1, size / chunks);

    let mut result: Vec<(Span, Span)> = Vec::new();

    let mut append = |a: Span, b: Span| {
      if a.0 == a.1 && b.0 == b.1 {
      } else {
        result.push((a, b));
      }
    };

    let mut i = 0;
    while i < size {
      let end = min(i + chunk_length, size);
      append((0, i), (end, size));

      i = end;
    }

    i = 0;
    while i < size {
      let end = min(i + chunk_length, size);
      append((i, end), (size, size));

      i = end;
    }

    result
  }
}
