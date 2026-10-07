use core::fmt;

use crate::enums::fragment_autocomplete_waypoint::FragmentAutocompleteWaypoint;

/// C++ `IFragmentAutocompleteReporter` 纯虚基类（`Analysis/include/Luau/FragmentAutocomplete.h`）：
/// 上报补全流程航点与被裁剪出的 fragment 源码。
///
/// 上报是单向「写出去」，实现方按需自带内部可变（`Cell`/`RefCell`），故接口只取
/// `&self`；这让 [`ReporterRef`] 可以用共享引用表达，从而保住 cpp 裸指针的 `Copy`
/// 语义，且没有任何 unsafe 解引用。
pub trait IFragmentAutocompleteReporter {
  fn report_waypoint(&self, waypoint: FragmentAutocompleteWaypoint);
  fn report_fragment_string(&self, fragment: &str);
}

/// 可空 reporter 引用，对应 C++ `IFragmentAutocompleteReporter* = nullptr`。
///
/// 这不是 `Option<&dyn _>` 的样板包装（review.md §4/§7 逐处评估结论）：它把
/// cpp 的 `if (reporter) reporter->reportWaypoint(...)` 空指针守卫收成一个
/// null-object，10 处上报点因此都是直白的 `reporter.report_*` 调用，内联成
/// `Option<&dyn _>` 字段就得在每处补一次判空；`Copy` 让它可以按值在
/// `fragment_autocomplete` → `typecheck_fragment` → `typecheck_fragment_` 多层
/// 间流转而无需 reborrow（借用检查器不必重新收窄生命周期）。
///
/// `dyn` 保留：reporter 由宿主注入（cpp 公开 API 形参），实现方集合运行期
/// 开放，无法 enum_dispatch 穷举。
#[derive(Clone, Copy)]
pub struct ReporterRef<'a>(Option<&'a dyn IFragmentAutocompleteReporter>);

impl<'a> ReporterRef<'a> {
  /// C++ `nullptr` 等价物。
  pub const NULL: Self = Self(None);

  /// 从 trait 对象的共享引用构造（只借用，不获取所有权）。全仓当前只用
  /// [`ReporterRef::NULL`]（补全航点上报是给宿主调试用的旁路口），本口是
  /// 该 pub API 唯一的非空构造入口，删掉即等于关掉这条上报链路。
  pub const fn new(reporter: &'a dyn IFragmentAutocompleteReporter) -> Self {
    Self(Some(reporter))
  }

  /// cpp `if (reporter) reporter->reportWaypoint(...)`：空引用即丢弃。
  pub fn report_waypoint(&self, waypoint: FragmentAutocompleteWaypoint) {
    if let Some(reporter) = self.0 {
      reporter.report_waypoint(waypoint);
    }
  }

  /// 同 [`ReporterRef::report_waypoint`]，cpp `reportFragmentString`。
  pub fn report_fragment_string(&self, fragment: &str) {
    if let Some(reporter) = self.0 {
      reporter.report_fragment_string(fragment);
    }
  }
}

impl fmt::Debug for ReporterRef<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("ReporterRef")
      .field(&self.0.is_some())
      .finish()
  }
}
