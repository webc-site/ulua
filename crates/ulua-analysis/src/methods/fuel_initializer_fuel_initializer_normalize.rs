use crate::records::{fuel_initializer::FuelInitializer, normalizer::Normalizer};

impl FuelInitializer {
  /// # Safety
  /// 调用方须保证 `normalizer` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn fuel_initializer_not_null_normalizer(&mut self, normalizer: *mut Normalizer) {
    self.normalizer = normalizer;
    self.initialized_fuel = unsafe { (*normalizer).initialize_fuel() };
  }
}
