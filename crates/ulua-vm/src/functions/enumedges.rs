use crate::{
  functions::{c_slice, enumedge::enum_edge},
  macros::{gcvalue::gcvalue, iscollectable::iscollectable},
  records::{enum_context::EnumContext, gc_object::GCObject},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `data` 必须指向连续 `size` 个可读的 `TValue`（即 `from` 对象某引用区，长度取自对象自身字段）；
/// `ctx`/`from`/`name` 满足 [`enum_edge`] 的契约（`name` 为 NUL 结尾字节串）。
/// 违反 `size` 与实区不符会越界读堆。cpp lgcdebug.cpp:770。
pub(crate) unsafe fn enumedges(
  ctx: *mut EnumContext,
  from: *mut GCObject,
  data: *mut TValue,
  size: usize,
  name: &[u8],
) {
  // Safety: 契约保证 `data..data+size` 为可读 TValue 区间，块内只读取值标签与 gcvalue 指针、不写对象
  unsafe {
    // Safety:data 指向 size 个 TValue（枚举边遍历只读，来源对象已在枚举前入队）。
    for val in c_slice(data, size) {
      if iscollectable!(val) {
        enum_edge(ctx, from, gcvalue!(val), name);
      }
    }
  }
}
