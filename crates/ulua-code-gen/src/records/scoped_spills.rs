use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::ir_reg_alloc_x_64::IrRegAllocX64};

#[derive(Debug)]
#[repr(C)]
pub struct ScopedSpills {
  pub(crate) owner: *mut IrRegAllocX64,
  pub(crate) start_spill_id: u32,
}

impl ScopedSpills {
  /// owner 视图重建的**单源收口**（`function_view`/`slots_mut` 同款形态）：
  /// `owner` 由构造点 `new(&mut IrRegAllocX64)` 以可变引用入表，非空、对齐且比本
  /// scope 长寿；`'s` 与 `&mut self` 解耦——数据活在宿主分配器而非本 scope 内，
  /// 与原逐语句 `&mut *self.owner` 的别名窗口逐位一致（空 owner 哨兵在 `drop`
  /// 早退处拦截，业务方法体保持 safe）。
  #[inline]
  fn owner_view<'s>(&mut self) -> &'s mut IrRegAllocX64 {
    // Safety: 契约即上方文档——构造点入表地址、比本 scope 长寿，Drop/方法体单线程
    // 串行即时消费，无并存别名。
    unsafe { &mut *self.owner }
  }
}

impl Drop for ScopedSpills {
  fn drop(&mut self) {
    if self.owner.is_null() {
      return;
    }

    // 视图重建单源于 `owner_view`；下方循环体无逐语句 unsafe。
    let owner = self.owner_view();
    let end_spill_id = owner.next_spill_id;
    let start_spill_id = self.start_spill_id;

    let mut i = 0;
    while i < owner.spills.len() {
      let spill = &owner.spills[i];

      // 恢复本 scope 内的 spill 不会产生新 spill。
      CODEGEN_ASSERT!(spill.spill_id < end_spill_id);

      if spill.spill_id >= start_spill_id {
        let inst_idx = spill.inst_idx as usize;
        // 恢复只回写该指令、不再新增 spill(见上断言)；instructions 视图经
        // `IrRegAllocX64::function_view` 的解耦契约重建，与 owner 的 restore 调用
        // 不同对象、互不别名。spill.inst_idx 是登记该 spill 时记录的合法指令
        // 下标(< len)，界内下标即安全。
        let inst = &mut owner.function_view().instructions[inst_idx];

        owner.restore(inst, true);
      } else {
        i += 1;
      }
    }
  }
}

impl ScopedSpills {
  /// cpp `ScopedSpills::ScopedSpills(IrRegAllocX64&)`（`IrRegAllocX64.cpp:760`）：
  /// 接线 owner 并把当前 spill 计数记为恢复下界。取代旧「字面量 + scoped 初始化」
  /// 两步写法（字面量字段全为死存储，随后即被覆盖）。
  pub fn new(owner: &mut IrRegAllocX64) -> Self {
    Self {
      owner: owner as *mut IrRegAllocX64,
      start_spill_id: owner.next_spill_id,
    }
  }
}
