use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_data::K_INVALID_INST_IDX, ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64,
  },
};

#[derive(Debug)]
#[repr(C)]
pub struct ScopedRegX64 {
  pub owner: *mut IrRegAllocX64,
  pub reg: RegisterX64,
}

impl ScopedRegX64 {
  pub fn alloc(&mut self, size: SizeX64) {
    CODEGEN_ASSERT!(self.reg == RegisterX64::NOREG);
    // Safety: owner 是构造 ScopedRegX64 时接线的寄存器分配器(IrRegAllocX64)裸指针, 非空且比该 scoped reg
    // 长寿; 分配器在单线程串行 lowering 中驱动, 此处 `&mut *self.owner` 取得的临时独占借用无并存别名。
    // 上方 CODEGEN_ASSERT 保证 reg==NOREG, 即不会对已持有寄存器的实例重复分配。
    let owner = unsafe { &mut *self.owner };
    self.reg = owner.alloc_reg(size, K_INVALID_INST_IDX);
  }

  pub fn free(&mut self) {
    CODEGEN_ASSERT!(self.reg != RegisterX64::NOREG, "ScopedRegX64::free");
    // Safety: owner 为构造时接线、非空且比本 scoped reg 长寿的分配器裸指针; 此临时 `&mut *self.owner`
    // 借用处于单线程串行 lowering, 无并存别名。上方 CODEGEN_ASSERT 保证 reg!=NOREG(确有所持才归还),
    // 随后立即置回 NOREG, 使 Drop 不再二次 free, 避免重复释放。
    unsafe { &mut *self.owner }.free_reg(self.reg);
    self.reg = RegisterX64::NOREG;
  }

  /// cpp `ScopedRegX64::ScopedRegX64(IrRegAllocX64&)`（`IrRegAllocX64.cpp:709`）：
  /// 先占位（`reg = noreg`），后续按条件 `alloc()` / `take()`。
  pub fn new(owner: &mut IrRegAllocX64) -> Self {
    Self {
      owner: owner as *mut IrRegAllocX64,
      reg: RegisterX64::NOREG,
    }
  }

  /// cpp `ScopedRegX64::ScopedRegX64(IrRegAllocX64&, SizeX64)`（`IrRegAllocX64.cpp:715`）：
  /// 构造即分配指定尺寸的寄存器，出作用域由 `Drop` 自动归还。
  pub fn with_size(owner: &mut IrRegAllocX64, size: SizeX64) -> Self {
    let mut this = Self::new(owner);
    this.alloc(size);
    this
  }

  pub fn release(&mut self) -> RegisterX64 {
    let tmp = self.reg;
    self.reg = RegisterX64::NOREG;
    tmp
  }

  pub fn take(&mut self, reg: RegisterX64) {
    CODEGEN_ASSERT!(self.reg.register_x_64_operator_eq(RegisterX64::NOREG));
    // Safety: owner 为构造时接线、非空且比本 scoped reg 长寿的分配器裸指针; `(*self.owner).take_reg` 取得的
    // 临时独占借用处于单线程串行 lowering, 无并存别名。上方 CODEGEN_ASSERT 保证 reg==NOREG(不覆盖已持有寄存器);
    // reg 参数由调用方提供的活寄存器标识, 结果写回 self.reg 交由此对象统一在 Drop/free 归还。
    self.reg = unsafe { (*self.owner).take_reg(reg, 0xffffffff) };
  }
}

impl Drop for ScopedRegX64 {
  fn drop(&mut self) {
    if self.reg != RegisterX64::NOREG {
      // Safety: owner 为构造时接线、非空且比本 scoped reg 长寿的分配器裸指针; free_reg 借用的临时
      // `&mut *self.owner` 处于单线程串行 lowering, 无并存别名。外层 reg!=NOREG 判定确保只对确实持有的
      // 寄存器归还一次, 与 alloc/take/free 的置位互斥, 不产生重复释放。
      unsafe { &mut *self.owner }.free_reg(self.reg);
    }
  }
}
