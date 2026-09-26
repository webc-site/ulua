//! Source: `tests/Fixture.h`（`TestFileResolver::source` 的共享句柄）
//!
//! cpp 侧的引用链是 `TestRequireSuggester::getNode → &resolver->source`
//! （`Fixture.cpp:141-144`），`TestRequireNode` 再持该地址遍历/查表
//! （`Fixture.cpp:115-134`）：suggester 与节点都**回指** `TestFileResolver` 的字段。
//! C++ 靠"`TestFileResolver` 拥有 suggester，且 suggester 不会比它活得久"这一
//! 隐含约定成立；Rust 里 suggester 存放在 `'static` 的 `Arc<dyn RequireSuggester>`
//! 中，无法借用宿主字段，用裸指针 + thread_local 布线则会随 fixture 析构/移动而悬垂。
//! 故改为引用计数共享：resolver 持唯一 `Rc`，suggester 持 `Weak`（`get_node` 时
//! `upgrade`，失败即无候选），派生出的节点持 `Rc` 克隆 —— 存活期由计数决定，
//! 不再有裸指针。

use alloc::{string::String, vec::Vec};
use std::cell::RefCell;

use ulua_analysis::type_aliases::module_name_type::ModuleName;
use ulua_common::collections::HashMap;

/// 模块名 → 源码；require 补全据此枚举候选模块。
///
/// 内部可变（`RefCell`）：写入方是测试用例（`fixture.file_resolver.source`），
/// 读取方是补全过程持有的 `Weak` 句柄，二者共享同一份表（对应 cpp 的 `&source`）。
#[derive(Debug, Default)]
pub struct SourceTable {
  entries: RefCell<HashMap<ModuleName, String>>,
}

impl SourceTable {
  /// cpp `source.insert({name, src})` / `source[name] = src`。
  /// 源码以 `impl Into<String>` 接收，测试侧可直接传 `&str` 字面量，免显式构造。
  pub fn insert(&self, name: impl Into<ModuleName>, source: impl Into<String>) -> Option<String> {
    self.entries.borrow_mut().insert(name.into(), source.into())
  }

  /// cpp `source.count(name)`：只判存在，不拷出值。
  pub fn contains(&self, name: &str) -> bool {
    self.entries.borrow().contains_key(name)
  }

  /// cpp `source.at(name)` 的读取；值按 cpp 的拷贝语义交出，`Ref` 随即释放。
  pub fn get(&self, name: &str) -> Option<String> {
    self.entries.borrow().get(name).cloned()
  }

  /// cpp `source.erase(name)`。
  pub fn remove(&self, name: &str) -> Option<String> {
    self.entries.borrow_mut().remove(name)
  }

  /// cpp `for (auto& entry : *allSources)` 的键快照：先取键再释放 `Ref`，
  /// 这样遍历期间构造子节点（会再次读表）不会与借用冲突。
  pub fn names(&self) -> Vec<ModuleName> {
    self.entries.borrow().keys().cloned().collect()
  }
}
