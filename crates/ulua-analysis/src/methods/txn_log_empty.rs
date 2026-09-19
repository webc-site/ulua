use alloc::{boxed::Box, vec::Vec};
/// Holds the process-lifetime empty `TxnLog` so it stays *reachable* from the
/// static (LeakSanitizer traces it). `TxnLog` contains raw pointers and so isn't
/// `Sync`; the empty log is immutable after construction and only ever read, so
/// sharing it across threads is sound. The previous `OnceLock<usize>` stored the
/// pointer as an integer (to dodge the `Sync` bound), which LSan cannot trace —
/// reported as a leak by the fuzz suite.
use core::ptr::null;
use core::ptr::null_mut;
use std::sync::OnceLock;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::txn_log::TxnLog;
struct SyncTxnLog(Box<TxnLog>);
// SAFETY: the empty log is never mutated after `get_or_init`; reads are safe to
// share. (It is also never freed — a deliberate process-lifetime singleton.)
// `OnceLock<T>: Sync` requires `T: Send + Sync`, so both are needed; `TxnLog`'s
// raw pointers make neither automatic.
unsafe impl Sync for SyncTxnLog {}
unsafe impl Send for SyncTxnLog {}

impl TxnLog {
  pub fn empty() -> *const TxnLog {
    static EMPTY_LOG: OnceLock<SyncTxnLog> = OnceLock::new();

    let wrapper = EMPTY_LOG.get_or_init(|| {
      let mut log = Box::new(TxnLog {
        type_var_changes: DenseHashMap::new(null()),
        type_pack_changes: DenseHashMap::new(null()),
        parent: null_mut(),
        owned_seen: Vec::new(),
        shared_seen: null_mut(),
        // Uses the inline `owned_seen` directly (see below), not a box.
        owned_seen_box: None,
        radioactive: false,
      });

      // Self-referential: `shared_seen` points at `owned_seen` inside the
      // box. Boxing pins the pointee's address, and moving the `Box` into
      // the `OnceLock` moves only the pointer, so this stays valid.
      log.shared_seen = &mut log.owned_seen;
      SyncTxnLog(log)
    });
    wrapper.0.as_ref() as *const TxnLog
  }
}
