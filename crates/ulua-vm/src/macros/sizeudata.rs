use core::mem::offset_of;

use crate::records::udata::Udata;

/// udata 数据块对齐粒度：`len > UDATA_ALIGN` 时向上取整到 UDATA_ALIGN 的倍数，
/// 否则原样（cpp `VM/src/ludata.h:17` `(len + 15) & ~15`）。
/// 建议后续把 `align_up` 上提到 ulua-common 供全仓复用。
const UDATA_ALIGN: usize = 16;

/// 向上取整到 `align` 的倍数：`(v + align - 1) & !(align - 1)` 的单点真相
/// （`align` 必须是 2 的幂，UDATA_ALIGN 由下方自检锁定）。
const fn align_up(value: usize, align: usize) -> usize {
  (value + align - 1) & !(align - 1)
}

/// 对齐粒度为 2 的幂且边界值与 cpp 字面式逐位一致：
/// 16 内原样、17 进 32、31 进 32、32 不动。
const _: () = assert!(
  UDATA_ALIGN & (UDATA_ALIGN - 1) == 0
    && align_up(0, UDATA_ALIGN) == 0
    && align_up(1, UDATA_ALIGN) == 16
    && align_up(16, UDATA_ALIGN) == 16
    && align_up(17, UDATA_ALIGN) == 32
    && align_up(31, UDATA_ALIGN) == 32
    && align_up(32, UDATA_ALIGN) == 32
);

#[inline]
pub const fn sizeudata(len: usize) -> usize {
  offset_of!(Udata, data)
    + if len > UDATA_ALIGN {
      align_up(len, UDATA_ALIGN)
    } else {
      len
    }
}
