#[macro_export]
macro_rules! VM_REG {
  ($i:expr, $l:expr, $base:expr) => {{
    let i = $i;
    let l = $l;
    let base = $base;
    ulua_common::LUAU_ASSERT!((i as u32) < ((*l).top.offset_from(base) as u32));
    // 返回 `*mut TValue`（StkId）；调用方无需再 `as *mut TValue` 转换，
    // 传 `*const TValue` 形参时由 mut->const 隐式强转覆盖。
    base.add(i as usize)
  }};
}

pub use VM_REG;
