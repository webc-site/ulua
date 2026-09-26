use core::fmt::{Debug, Formatter, Result};

use crate::records::file_resolver::FileResolver;

/// C++ `ErrorConverter` 的可空 `const FileResolver*` 成员：错误串生成只用到
/// `&self` 方法（只读），故以共享引用 `&'a dyn FileResolver` 建模，`None` 即空。
///
/// 此处 `dyn` 保留：唯一供源是 `Frontend::file_resolver_ref()`——已在
/// `FileResolver` 运行期开放边界（宿主注入、跨 crate 多实现方）处完成类型
/// 擦除，本结构只是边界的下游消费方；对其泛型化无处可选具体类型，只会把
/// 同一 `dyn` 换个姿势重新引入。
#[derive(Clone, Default)]
pub struct ErrorConverter<'a> {
  pub(crate) file_resolver: Option<&'a dyn FileResolver>,
}

/// This record represents the C++ `ErrorConverter` struct used as a visitor/functor
/// to convert `TypeErrorData` variants into human-readable strings.
///
/// In Rust, the `operator()` overloads are translated as inherent methods or
/// a single dispatch method. Since the schedule identifies this as a `record`,
/// we only emit the struct definition here. The implementation of the conversion
/// logic for each error variant will be provided in separate `impl` blocks.
impl<'a> ErrorConverter<'a> {
  pub(crate) fn new(file_resolver: Option<&'a dyn FileResolver>) -> Self {
    Self { file_resolver }
  }

  /// cpp `ErrorConverter::fileResolver`（可为 null）的受控读取：字段本身就是
  /// 带生命周期的共享引用，此处仅原样返回，无 unsafe。
  pub(crate) fn file_resolver_ref(&self) -> Option<&'a dyn FileResolver> {
    self.file_resolver
  }
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
