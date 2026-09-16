use core::fmt;

use crate::enums::fragment_autocomplete_waypoint::FragmentAutocompleteWaypoint;
pub trait IFragmentAutocompleteReporter {
  fn report_waypoint(&mut self, waypoint: FragmentAutocompleteWaypoint);
  fn report_fragment_string(&mut self, fragment: &str);
}

/// 可空 reporter 引用，对应 C++ `IFragmentAutocompleteReporter* = nullptr`。
/// `Copy` 使其可在多层调用间按值流转；借用有效性由调用方维持，
/// 唯一的 `unsafe` 解引用集中在此类型的两个方法内。
#[derive(Clone, Copy)]
pub struct ReporterRef<'a>(Option<*mut (dyn IFragmentAutocompleteReporter + 'a)>);

impl<'a> ReporterRef<'a> {
  /// C++ `nullptr` 等价物。
  pub const NULL: Self = Self(None);

  /// 从 trait 对象可变引用构造（借用，不获取所有权）。
  pub fn new(reporter: &mut (dyn IFragmentAutocompleteReporter + 'a)) -> Self {
    Self(Some(reporter))
  }

  pub fn is_null(&self) -> bool {
    self.0.is_none_or(|p| p.is_null())
  }

  pub fn report_waypoint(&mut self, waypoint: FragmentAutocompleteWaypoint) {
    if let Some(reporter) = self.0 {
      // SAFETY: 构造方保证引用在本次调用期间有效（与 C++ 裸指针借用语义一致）
      unsafe { (*reporter).report_waypoint(waypoint) };
    }
  }

  pub fn report_fragment_string(&mut self, fragment: &str) {
    if let Some(reporter) = self.0 {
      // SAFETY: 同上
      unsafe { (*reporter).report_fragment_string(fragment) };
    }
  }
}

impl fmt::Debug for ReporterRef<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("ReporterRef")
      .field(&!self.is_null())
      .finish()
  }
}
