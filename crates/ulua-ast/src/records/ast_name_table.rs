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

// Safety: 唯一非 auto 字段是 `allocator`（指向声明了 Send+Sync 的 Allocator）；`DenseHashSet<Entry,EntryHash>`
// 只存 plain data，其 AstName 字节均指向上面 allocator 的页。转移本表等价于转移 allocator，符合其单次访问约束，故 Send 成立。
unsafe impl Send for AstNameTable {}
// Safety: 共享 `&AstNameTable` 仅暴露 `&Allocator`（其 Sync 见 records/allocator.rs 契约）与对 plain
// entry 数据的只读访问，不含可交出重叠 `&mut` 的途径，故 Sync 成立（同 Send 的单线程访问纪律）。
unsafe impl Sync for AstNameTable {}
