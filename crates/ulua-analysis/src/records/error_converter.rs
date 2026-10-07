use core::fmt::{Debug, Formatter, Result};

use crate::records::file_resolver::FileResolver;

/// C++ `ErrorConverter`：把 `TypeErrorData` 各变体转成人读字符串的 visitor/functor。
/// cpp 的 `operator()` 重载在 Rust 侧是各 `impl` 块里的固有方法，本文件只声明结构。
///
/// 可空 `const FileResolver*` 成员：错误串生成只用到 `&self` 方法（只读），故以
/// 共享引用 `&'a dyn FileResolver` 建模，`None` 即空。字段直接暴露
/// （review.md §7「内部结构体直接暴露字段，别套 getter」），`new` /
/// `file_resolver_ref` 两个原样转发的样板访问器已删。
///
/// 此处 `dyn` 保留：唯一供源是 [`Frontend`](crate::records::frontend::Frontend)
/// 的 `file_resolver`（已在 `FileResolver` 运行期开放边界——宿主注入、跨 crate
/// 多实现方——处完成类型擦除），本结构只是边界的下游消费方；对其泛型化无处
/// 可选具体类型，只会把同一 `dyn` 换个姿势重新引入。
#[derive(Clone, Default)]
pub struct ErrorConverter<'a> {
  pub(crate) file_resolver: Option<&'a dyn FileResolver>,
}

// 手写 Debug 以保持与旧的裸指针 derive 输出逐字一致（`Some(0x…)` 指针格式）。
impl Debug for ErrorConverter<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.file_resolver {
      Some(resolver) => f
        .debug_struct("ErrorConverter")
        .field("file_resolver", &format_args!("Some({:p})", resolver))
        .finish(),
      None => f
        .debug_struct("ErrorConverter")
        .field("file_resolver", &"None")
        .finish(),
    }
  }
}
