//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TxnLog.cpp:187:occurs`
//! Source: `Analysis/src/TxnLog.cpp:187-217` (hand-ported)

use crate::{
  functions::follow_once::follow_once, records::txn_log::TxnLog, type_aliases::type_id::TypeId,
};

/// We must take extra care not to replace a type with a BoundType to itself. We
/// check each BoundType along the chain.
///
/// This function returns true if any of the bound types pointed at by `needle`
/// point at `haystack`.
pub fn occurs_txn_log_type_id_type_id(log: &mut TxnLog, needle: TypeId, haystack: TypeId) -> bool {
  let mut tortoise: TypeId = needle;
  let mut hare: TypeId = needle;

  loop {
    if tortoise == haystack {
      return true;
    }

    let g = follow_once(log, tortoise);
    if g.is_null() {
      return false;
    }
    tortoise = g;

    // Cycle detection: The hare steps twice for each step that the tortoise takes.
    // If ever the two meet, it can only be because the track is cyclic.
    // When we hit the end of the chain, hare becomes nullptr.
    if !hare.is_null() {
      hare = follow_once(log, hare);
      if !hare.is_null() {
        hare = follow_once(log, hare);

        if hare == tortoise {
          return true;
        }
      }
    }
  }
}
