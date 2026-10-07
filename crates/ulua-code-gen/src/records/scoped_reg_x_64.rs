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
  /// owner 视图重建的**单源收口**（同 `ScopedSpills::owner_view` 形态）：
  /// `owner` 由构造点 `new(&mut IrRegAllocX64)` 以可变引用入表，非空、对齐且比本
  /// scoped reg 长寿；分配器在单线程串行 lowering 中驱动，借用即时消费。空 owner
  /// 不可能出现（构造点唯一），上方各 `CODEGEN_ASSERT` 保证不与重复分配/释放交叠。
  #[inline]
  fn owner_view<'s>(&mut self) -> &'s mut IrRegAllocX64 {
    // Safety: 契约即上方文档。
    unsafe { &mut *self.owner }
  }

  pub fn alloc(&mut self, size: SizeX64) {
    CODEGEN_ASSERT!(self.reg == RegisterX64::NOREG);
    // 视图重建单源于 `owner_view`；上方断言保证不会对已持有寄存器的实例重复分配。
    let owner = self.owner_view();
    self.reg = owner.alloc_reg(size, K_INVALID_INST_IDX);
  }

  pub fn free(&mut self) {
    CODEGEN_ASSERT!(self.reg != RegisterX64::NOREG, "ScopedRegX64::free");
    // 上方断言保证确有所持才归还；随后立即置回 NOREG，使 Drop 不再二次 free。
    let reg = self.reg;
    self.owner_view().free_reg(reg);
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
    CODEGEN_ASSERT!(self.reg.same_layout(RegisterX64::NOREG));
    // 视图重建单源于 `owner_view`（借用处于单线程串行 lowering，无并存别名）；上方
    // CODEGEN_ASSERT 保证不覆盖已持有寄存器；结果写回 self.reg 由此对象统一在 Drop/free 归还。
    let owner = self.owner_view();
    self.reg = owner.take_reg(reg, 0xffffffff);
  }
}

impl Drop for ScopedRegX64 {
  fn drop(&mut self) {
    if self.reg != RegisterX64::NOREG {
      // 外层 reg!=NOREG 判定确保只对确实持有的寄存器归还一次，与 alloc/take/free
      // 的置位互斥，不产生重复释放；视图重建单源于 `owner_view`。
      let reg = self.reg;
      self.owner_view().free_reg(reg);
    }
  }
}
