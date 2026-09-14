use alloc::boxed::Box;
use core::fmt::Debug;

use crate::{records::require_node::RequireNode, type_aliases::module_name_type::ModuleName};

/// C++ `Luau::RequireSuggester` 虚基类（`Analysis/include/Luau/FileResolver.h`）
/// 的 Rust 化：虚函数接口 → trait，实现方以 `dyn RequireSuggester` 传递，
/// 替代原先 `#[repr(C)]` 手写 vtable + `unsafe fn` 指针。
///
/// `Debug` 约束：实现方会作为 `Arc<dyn RequireSuggester>` 挂在测试 resolver 等
/// 带 `#[derive(Debug)]` 的结构上。
pub trait RequireSuggester: Debug {
  /// `getNode`：按模块名给出 suggester 节点；C++ 纯虚。
  fn get_node(&self, name: &ModuleName) -> Option<Box<dyn RequireNode>>;
}
