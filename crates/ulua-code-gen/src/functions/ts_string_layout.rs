//! `ulua_vm::records::t_string::tstring` 的跨 crate 只读投影。
//!
//! ulua-vm 的 `hash` 字段是 `pub(crate)`，ulua-code-gen 不可见（`len` 为 `pub` 可直读）；
//! 本模块按相同 `#[repr(C)]` 布局镜像读取。若 `tstring` 布局变更，须同步此文件。

use ulua_vm::records::t_string::tstring;

#[repr(C)]
struct TsStringHeader {
  // GCheader（tt/marked/memcat），镜像读取不需要字段名
  _hdr: [u8; 3],
  _padding1: [u8; 1],
  _atom: i16,
  _padding2: [u8; 2],
  _next: *mut tstring,
  hash: u32,
  // 占位保持 hash 之后的偏移与 tstring 一致；本 crate 不经镜像读 len。
  _len: u32,
  _data: [u8; 1],
}

/// # Safety
/// `ts` 必须指向存活对象。
pub(crate) unsafe fn ts_hash(ts: *const tstring) -> u32 {
  // Safety: TsStringHeader 是 tstring 的 #[repr(C)] 布局镜像(见模块注释), hash 偏移与 tstring 完全一致;
  // ts 由调用方保证非空/对齐/存活, 故按镜像布局重解释后读取 hash 字段落在同一对象内、对齐正确, 只读安全。
  unsafe { (*(ts as *const TsStringHeader)).hash }
}
