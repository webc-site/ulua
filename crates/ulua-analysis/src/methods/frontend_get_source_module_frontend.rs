//! cpp `SourceModule* Frontend::getSourceModule(const ModuleName&)`
//! （`Analysis/src/Frontend.cpp`）的 Rust 形态。
//!
//! 原实现两版都以裸指针返回、用 `null_mut()` 当「模块不存在」哨兵，且只读版
//! 还要先把 `&self` 转铸成 `*mut Frontend` 才能复用可变版（同一对象的共享
//! 借用被当作独占借用使用，属指针别名 UB 窗口）。本次收口（步⑤ 可变版 +
//! 挂账② 只读版）：
//!
//! - 两版统一返回 [`Option<Handle<SourceModule>>`]——`Option` 承载可空性，
//!   `Handle` 承载「宿主表内实例的别名句柄」语义（契约见 `records/arena_handle.rs`），
//!   调用点不再触碰裸指针、null 哨兵与非空证明。

use alloc::sync::Arc;

use crate::{
  records::{arena_handle::Handle, frontend::Frontend, source_module::SourceModule},
  type_aliases::module_name_type::ModuleName,
};

impl Frontend {
  /// 可变形态：`source_modules` 表内该模块的别名句柄，模块未检查/未保留时为
  /// `None`（取代原 `null_mut()` 哨兵）。
  ///
  /// 句柄不绑定 `&mut self` 生命周期，与原裸指针返回值的借用检查行为同构
  /// （否则调用点 `frontend.module_resolver.get_module(..)` 一类后续访问无法
  /// 与已取出的模块共存），解引用契约集中在 `records/arena_handle.rs`。
  /// 指针直接取自 `Arc::as_ptr`（非经共享借用转化），保持其堆分配的原始 provenance。
  pub fn get_source_module_mut(
    &mut self,
    module_name: &ModuleName,
  ) -> Option<Handle<SourceModule>> {
    self
      .source_modules
      .get_mut(module_name)
      .map(|module| unsafe { Handle::from_raw(Arc::as_ptr(module) as *mut SourceModule) })
  }

  /// 只读形态（cpp 同一函数的 `const` 用法）：与可变版同一 `Option<Handle>`
  /// 交付——`Handle` 派生自共享引用，下游只读消费（`get`）；需要写穿语义的
  /// 调用点走 [`Frontend::get_source_module_mut`]。
  pub fn get_source_module(&self, module_name: &ModuleName) -> Option<Handle<SourceModule>> {
    self
      .source_modules
      .get(module_name)
      .map(|module| Handle::from_ref(&**module))
  }
}
