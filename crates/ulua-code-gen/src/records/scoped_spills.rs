use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::ir_reg_alloc_x_64::IrRegAllocX64};

#[derive(Debug)]
#[repr(C)]
pub struct ScopedSpills {
  pub(crate) owner: *mut IrRegAllocX64,
  pub(crate) start_spill_id: u32,
}

impl Drop for ScopedSpills {
  fn drop(&mut self) {
    if self.owner.is_null() {
      return;
    }

    // Safety: 上方 self.owner.is_null() 已提前返回, owner 非空且为构造期接线、比本 scope 长寿的合法
    // *mut IrRegAllocX64; Drop 阶段单线程独占, 派生 &mut 重建可变借用无并发别名。
    let owner = unsafe { &mut *self.owner };
    let end_spill_id = owner.next_spill_id;

    let mut i = 0;
    while i < owner.spills.len() {
      let spill = &owner.spills[i];

      // 恢复本 scope 内的 spill 不会产生新 spill。
      CODEGEN_ASSERT!(spill.spill_id < end_spill_id);

      if spill.spill_id >= self.start_spill_id {
        let inst_idx = spill.inst_idx as usize;
        // Safety: owner.function 为接线非空/长寿 *mut IrFunction; spill.inst_idx 是先前登记该 spill 时记录的
        // 合法指令下标(< instructions.len()), as_mut_ptr().add(inst_idx) 落在缓冲界内, 解引用得存活 IrInst 的
        // &mut。恢复只回写该指令、不再新增 spill(见上断言), owner 持有的是另一对象, 二者不相互别名。
        let inst = unsafe { &mut *(*owner.function).instructions.as_mut_ptr().add(inst_idx) };

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
