// 宏体内 unsafe 为 FFI 语义必需:供安全上下文中的调用点使用;
// 在 unsafe 上下文中展开时会报 unused_unsafe(多展开点重复计数),属已知误报。
#[macro_export]
macro_rules! vm_protect {
  ($l:expr, $pc:expr, $base:expr, $x:expr) => {
    (*(*$l).ci).savedpc = $pc;
    {
      $x;
    };
    $base = (*$l).base;
  };
  // 不需要同步 base 的调用点(赋值后 base 不再读取)使用三参形式,避免死赋值
  ($l:expr, $pc:expr, $x:expr) => {
    (*(*$l).ci).savedpc = $pc;
    {
      $x;
    };
  };
}

pub use vm_protect;
