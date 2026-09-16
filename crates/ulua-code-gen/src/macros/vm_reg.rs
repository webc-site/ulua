// 宏体内 unsafe 为 FFI 语义必需:供安全上下文中的调用点使用;
// 在 unsafe 上下文中展开时会报 unused_unsafe(多展开点重复计数),属已知误报。
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
