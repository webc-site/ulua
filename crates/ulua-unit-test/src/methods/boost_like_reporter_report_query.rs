use core::{ffi::c_void, slice::from_raw_parts};

use crate::records::boost_like_reporter::BoostLikeReporter;
impl BoostLikeReporter {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn report_query(&mut self, qd_num_data: u32, qd_data: *const *const c_void) {
    // TestCaseData 布局在翻译上下文中不可见，无法安全解引用提取
    // m_test_suite / m_name（C++ 实现会逐条打印），故此为 no-op 桩。
    for _tc_ptr in unsafe { from_raw_parts(qd_data, qd_num_data as usize) } {}

    // C++ 还会向 stderr 打印 "Found %d tests."，因循环体无法实现而省略。
  }
}
