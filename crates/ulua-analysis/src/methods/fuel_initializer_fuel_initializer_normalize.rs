use crate::records::{fuel_initializer::FuelInitializer, normalizer::Normalizer};

impl FuelInitializer {
  /// # Safety
  /// `normalizer` 须指向调用期间存活的 `Normalizer`（C++ `FuelInitializer(NotNull<Normalizer*>)`
  /// 契约）。
  pub unsafe fn fuel_initializer_not_null_normalizer(&mut self, normalizer: *mut Normalizer) {
    self.normalizer = normalizer;
    // Safety: 全部调用点均在 Normalizer 方法内以 `self as *mut Normalizer` 实参
    // （直译 C++ `FuelInitializer fi{NotNull{this}}`）——非空、对齐且即调用者
    // 本身，在本语句期间必然存活；initialize_fuel 是其正常方法调用。
    self.initialized_fuel = unsafe { (*normalizer).initialize_fuel() };
  }

  pub fn fuel_initializer_fuel_initializer_destructor(&mut self) {
    if self.initialized_fuel {
      // Safety: initialized_fuel 仅在 fuel_initializer_not_null_normalizer 中置位，
      // 且同一调用同步把 self.normalizer 接为该调用者 Normalizer 的写句柄——本分支
      // 成立即蕴含指针非空；FuelInitializer 都是作为栈上守卫先于其宿主 Normalizer
      // 析构（上方各调用点），目标对象此刻仍存活，clear_fuel 与 initialize_fuel
      // 配对。
      unsafe { (*self.normalizer).clear_fuel() };
      self.initialized_fuel = false;
    }
  }
}
