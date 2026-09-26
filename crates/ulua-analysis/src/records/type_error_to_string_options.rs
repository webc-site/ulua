use core::fmt::{Debug, Formatter, Result};

use crate::records::file_resolver::FileResolver;

/// C++ `TypeErrorToStringOptions` 的可空 `const FileResolver* fileResolver`：
/// 以带生命周期的共享引用建模，`None` 即空。`dyn` 保留：`FileResolver`
/// 实现方集合运行期开放（宿主跨 crate 注入），编译期不可穷举。
#[derive(Clone, Copy, Default)]
pub struct TypeErrorToStringOptions<'a> {
  pub file_resolver: Option<&'a dyn FileResolver>,
}

// 手写 Debug 以保持与旧的裸指针 derive 输出逐字一致（`Some(0x…)` 指针格式）。
impl Debug for TypeErrorToStringOptions<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.file_resolver {
      Some(resolver) => f
        .debug_struct("TypeErrorToStringOptions")
        .field("file_resolver", &format_args!("Some({:p})", resolver))
        .finish(),
      None => f
        .debug_struct("TypeErrorToStringOptions")
        .field("file_resolver", &"None")
        .finish(),
    }
  }
}
