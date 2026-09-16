use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{allocator::Allocator, entry::Entry, entry_hash::EntryHash};

// `Entry` and `EntryHash` are their own record items (`records::entry`,
// `records::entry_hash`); the table just stores them.
#[repr(C)]
#[derive(Debug)]
pub struct AstNameTable {
  pub(crate) data: DenseHashSet<Entry, EntryHash>,
  pub(crate) allocator: *mut Allocator,
}

unsafe impl Send for AstNameTable {}
unsafe impl Sync for AstNameTable {}
