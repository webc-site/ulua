use crate::records::{
  arena_handle::alias, fuel_initializer::FuelInitializer, normalizer::Normalizer,
};

impl FuelInitializer {
  /// # Safety
  /// `normalizer` 须指向调用期间存活的 `Normalizer`（C++ `FuelInitializer(NotNull<Normalizer*>)`
  /// 契约）。
  pub(crate) unsafe fn fuel_initializer_not_null_normalizer(
    &mut self,
    normalizer: *mut Normalizer,
  ) {
    self.normalizer = normalizer;
    self.initialized_fuel = alias(normalizer).initialize_fuel();
  }

  pub fn fuel_initializer_fuel_initializer_destructor(&mut self) {
    if self.initialized_fuel {
      // initialized_fuel 仅在 fuel_initializer_not_null_normalizer 中置位，本分支
      // 成立即蕴含 self.normalizer 非空；FuelInitializer 都是作为栈上守卫先于其
      // 宿主 Normalizer 析构（上方各调用点），clear_fuel 与 initialize_fuel 配对。
      alias(self.normalizer).clear_fuel();
      self.initialized_fuel = false;
    }
  }
}
