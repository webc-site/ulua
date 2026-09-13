//! Node: `cxx:Record:Luau.Repl.CLI:CLI/src/Counters.cpp:19:function_counters`
//! Source: `CLI/src/Counters.cpp`
//! Graph edges:
//! - declared_by: source_file CLI/src/Counters.cpp
//! - source_includes:
//!   - includes -> source_file CLI/include/Luau/Counters.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGenOptions.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file VM/include/lua.h
//! - incoming:
//!   - declares <- source_file CLI/src/Counters.cpp
//!   - type_ref <- record ModuleCounters (CLI/src/Counters.cpp)
//!   - type_ref <- function countersValueCallback (CLI/src/Counters.cpp)
//!   - type_ref <- function countersDump (CLI/src/Counters.cpp)
//! - outgoing:
//!   - type_ref -> record DenseHashMap (Common/include/Luau/DenseHash.h)
//!   - type_ref -> record LineCounters (CLI/src/Counters.cpp)
//!   - translates_to -> rust_item FunctionCounters

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
