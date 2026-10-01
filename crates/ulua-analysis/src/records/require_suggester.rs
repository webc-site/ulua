use core::fmt::Debug;

use crate::{records::require_node::RequireNode, type_aliases::module_name_type::ModuleName};

/// C++ `Luau::RequireSuggester` 虚基类（`Analysis/include/Luau/FileResolver.h`）
/// 的 Rust 化：虚函数接口 → trait，实现方以 `dyn RequireSuggester` 传递，
/// 替代原先 `#[repr(C)]` 手写 vtable + `unsafe fn` 指针。
///
/// `Debug` 约束：实现方会作为 `Arc<dyn RequireSuggester>` 挂在测试 resolver 等
/// 带 `#[derive(Debug)]` 的结构上。
pub trait RequireSuggester: Debug {
  /// cpp `getNode`：按模块名定位根节点，存在即以借用方式交给 `visit`（对应
  /// cpp 返回 `unique_ptr<RequireNode>`，Rust 侧免掉这次堆装箱），否则不调用。
  /// `dyn` 保留：trait 面向跨 crate 开放实现方须保持对象安全，泛形参会破坏
  /// `Arc<dyn RequireSuggester>` 布线（见 [`RequireNode`] 同注）。
  fn with_node(&self, name: &ModuleName, visit: &mut dyn FnMut(&dyn RequireNode));
}
