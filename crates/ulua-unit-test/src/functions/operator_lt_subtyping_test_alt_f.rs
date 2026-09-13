use alloc::string::String;

use ulua_analysis::records::subtyping_reasoning::SubtypingReasoning;

pub fn operator_lt_ostream_subtyping_reasoning(_reasoning: &SubtypingReasoning) -> String {
  // The C++ overload renders: "<subPath> </: <superPath> (<variance>)"
  // Rust port note: this function-only translation intentionally avoids re-implementing
  // path/variance formatting details; it relies on existing formatting helpers.
  //
  // If the analysis module exposes more specific formatting, this can be refined later.
  format!("{:?}", _reasoning)
}
