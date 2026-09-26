use crate::records::{global_state::global_State, scoped_set_gc_threshold::ScopedSetGcThreshold};

impl ScopedSetGcThreshold {
  /// # Safety
  ///
  /// `global` 必须为空或指向存活 `global_State`（守卫存活期内不被释放），否则 Drop 时
  /// 回写 `gc_threshold` 会解引用悬垂指针。
  pub(crate) unsafe fn scoped_set_gc_threshold_global_state_usize(
    &mut self,
    global: *mut global_State,
    new_threshold: usize,
  ) {
    self.global = global;
    // 一次 as_mut 判空并绑定引用，替换两处裸指针解引用样板
    // Safety: 契约保证 global 为空或指向存活 global_State，as_mut 判空后才读写 gc_threshold
    if let Some(global) = unsafe { global.as_mut() } {
      self.original_threshold = global.gc_threshold;
      global.gc_threshold = new_threshold;
    }
  }
}

impl Drop for ScopedSetGcThreshold {
  fn drop(&mut self) {
    // global 为空表示未启用（守卫未被赋值）；一次 as_mut 判空并绑定引用
    // Safety: self.global 只会是构造时契约校验过的指针或空，Drop 时其指对对象仍存活（守卫寿命约束）
    unsafe {
      if let Some(global) = self.global.as_mut() {
        global.gc_threshold = self.original_threshold;
      }
    }
  }
}
