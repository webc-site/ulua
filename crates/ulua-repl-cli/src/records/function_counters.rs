//! Source: `CLI/src/Counters.cpp`

use alloc::{collections::BTreeMap, string::String};

use crate::records::line_counters::LineCounters;

// Faithful port of Counters.cpp's:
//     struct FunctionCounters {
//         std::string name;
//         Luau::DenseHashMap<int, LineCounters> counters{-1};
//     };
// The C++ DenseHashMap is keyed by line number and later sorted by line for
// output; a BTreeMap captures the same line→LineCounters mapping in sorted
// order, so counters_dump can iterate it directly.
#[derive(Debug, Clone, Default)]
pub struct FunctionCounters {
  pub name: String,
  pub counters: BTreeMap<i32, LineCounters>,
}
