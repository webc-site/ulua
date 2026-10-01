use core::ptr::NonNull;

use ulua_analysis::records::scope::Scope;

use crate::{functions::raw_handle::raw_handle, records::normalize_fixture::NormalizeFixture};
impl NormalizeFixture {
  /// 全局 scope 句柄（review.md §2：可空返回值用 `Option`，不用 null 哨兵）。
  /// `get_frontend()` 会把 `global_scope` 填充为 `Some`，故实际调用方总是拿到
  /// `Some`；`None` 仅在夹具未初始化的理论路径出现，调用方据此解包而非收到
  /// 一个可空裸指针后继续解引用。裸句柄仍经 `raw_handle` 单一转换点取自
  /// `Arc`，包进 `NonNull` 后非空性由类型表达。
  pub fn get_global_scope(&mut self) -> Option<NonNull<Scope>> {
    self.get_frontend();
    self
      .global_scope
      .as_ref()
      .and_then(|scope| NonNull::new(raw_handle(scope)))
  }
}
