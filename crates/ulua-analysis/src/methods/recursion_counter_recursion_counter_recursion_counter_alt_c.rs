use crate::records::recursion_counter::RecursionCounter;
impl RecursionCounter {
  /// # Safety
  /// 调用方须保证 `count` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// C++ `RecursionCounter::RecursionCounter(int* count) : count(count) { ++(*count); }`
  /// (`Analysis/src/RecursionCounter.cpp:14`).
  pub unsafe fn recursion_counter_i32(count: *mut i32) -> RecursionCounter {
    unsafe {
      *count += 1;
    }
    RecursionCounter { count }
  }
}
