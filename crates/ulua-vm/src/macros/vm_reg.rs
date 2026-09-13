#[macro_export]
macro_rules! VM_REG {
  ($i:expr, $l:expr, $base:expr) => {{
    let i = $i;
    let l = $l;
    let base = $base;
    ulua_common::LUAU_ASSERT!((i as u32) < ((*l).top.offset_from(base) as u32));
    &mut *base.add(i as usize)
  }};
}

pub use VM_REG;
