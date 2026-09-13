#[macro_export]
macro_rules! VM_KV {
  ($i:expr, $cl:expr, $k:expr) => {{
    let i = $i;
    let cl = $cl;
    let k = $k;
    ulua_common::LUAU_ASSERT!((i as u32) < ((*(*cl).inner.l.p).sizek as u32));
    &mut *k.add(i as usize)
  }};
}

pub use VM_KV;
