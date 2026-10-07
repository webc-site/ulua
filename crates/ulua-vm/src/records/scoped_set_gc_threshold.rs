use core::ptr::NonNull;

use crate::records::global_state::global_State;

/// GC 阈值冻结 RAII 守卫（cpp `lvmload.cpp:801` `ScopedSetGCThreshold pauseGC{L->global, SIZE_MAX}`）。
///
/// DELIBERATE DEVIATION（review.md §2 规则 1「null 只是占位形态、无缺席语义」+ §3「出参接线
/// → 构造即接线」）：cpp 允许「默认构造再赋值」，于是留有 `global == nullptr` 的未接线合法
/// 形态；本 Rust 移植唯一构造点始终传入非空 `global`（cpp 里 `L->global` 从不为空），该空
/// 形态无人消费，故 `global` 收为 `NonNull<global_State>`——守卫存续期恒指向存活
/// `global_State`（由 `arm` 的调用序契约承载：守卫寿命不得超过所借 `global_State`）。
/// 逐字节布局与 repr(C) 保持原形；语义与 cpp 逐点一致：装臂时保存原阈值并冻结新值，
/// Drop 时恢复。
#[derive(Debug)]
#[repr(C)]
pub struct ScopedSetGcThreshold {
  global: NonNull<global_State>,
  original_threshold: usize,
}

impl ScopedSetGcThreshold {
  /// 装臂：把 `global.gc_threshold` 冻结为 `new_threshold`，保存的原值在 Drop 时恢复。
  ///
  /// 入参以 `&mut global_State` 承载非空/对齐/独占可写前提（一次短借，止于本调用；
  /// 守卫只留地址不留借用，与 cpp 裸指针同款别名形态）。调用序契约：守卫须先于所臂
  /// `global_State` 销毁（典型形态即同作用域栈上守卫，对应 cpp RAII）。
  pub(crate) fn arm(global: &mut global_State, new_threshold: usize) -> Self {
    let original_threshold = global.gc_threshold;
    global.gc_threshold = new_threshold;
    Self {
      global: NonNull::from(global),
      original_threshold,
    }
  }
}

impl Drop for ScopedSetGcThreshold {
  fn drop(&mut self) {
    // SAFETY: `arm` 的类型前提（`&mut global_State`）与调用序契约保证本地址指向的
    // global_State 在守卫整个存续期内存活且无并发写者；此处仅恢复装臂前保存的阈值
    unsafe { self.global.as_mut().gc_threshold = self.original_threshold };
  }
}
