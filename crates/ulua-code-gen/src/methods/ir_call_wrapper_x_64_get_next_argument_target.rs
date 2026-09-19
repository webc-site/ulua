use crate::{
  enums::{abix_64::ABIX64, category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_call_wrapper_x_64::IrCallWrapperX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

// Bytes for register 'home' locations that can be used by callees under Windows ABI.
const K_STACK_REG_HOME_STORAGE: i32 = 4 * 8;

pub(crate) const fn xmm(i: u8) -> RegisterX64 {
  RegisterX64 {
    bits: (i << RegisterX64::INDEX_SHIFT) | SizeX64::Xmmword as u8,
  }
}

// static const std::array<OperandX64, 6> kWindowsGprOrder
pub(crate) fn windows_gpr_order() -> [OperandX64; 6] {
  [
    OperandX64::reg(RegisterX64::RCX),
    OperandX64::reg(RegisterX64::RDX),
    OperandX64::reg(RegisterX64::R8),
    OperandX64::reg(RegisterX64::R9),
    OperandX64::mem(
      SizeX64::None,
      RegisterX64::NOREG,
      1,
      RegisterX64::RSP,
      K_STACK_REG_HOME_STORAGE,
    ),
    OperandX64::mem(
      SizeX64::None,
      RegisterX64::NOREG,
      1,
      RegisterX64::RSP,
      K_STACK_REG_HOME_STORAGE + 8,
    ),
  ]
}

// static const std::array<OperandX64, 6> kSystemvGprOrder
pub(crate) fn systemv_gpr_order() -> [OperandX64; 6] {
  [
    OperandX64::reg(RegisterX64::RDI),
    OperandX64::reg(RegisterX64::RSI),
    OperandX64::reg(RegisterX64::RDX),
    OperandX64::reg(RegisterX64::RCX),
    OperandX64::reg(RegisterX64::R8),
    OperandX64::reg(RegisterX64::R9),
  ]
}

// static const std::array<OperandX64, 4> kXmmOrder
pub(crate) fn xmm_order() -> [OperandX64; 4] {
  [
    OperandX64::reg(xmm(0)),
    OperandX64::reg(xmm(1)),
    OperandX64::reg(xmm(2)),
    OperandX64::reg(xmm(3)),
  ]
}

impl IrCallWrapperX64 {
  pub fn get_next_argument_target(&self, size: SizeX64) -> OperandX64 {
    if size == SizeX64::Xmmword {
      CODEGEN_ASSERT!((self.xmm_pos as usize) < xmm_order().len());
      return xmm_order()[self.xmm_pos as usize];
    }

    let gpr_order = if unsafe { (*self.build).abi } == ABIX64::WINDOWS {
      windows_gpr_order()
    } else {
      systemv_gpr_order()
    };

    CODEGEN_ASSERT!((self.gpr_pos as usize) < gpr_order.len());
    let mut target = gpr_order[self.gpr_pos as usize];

    // Keep requested argument size
    if target.cat == CategoryX64::Reg {
      target.base = RegisterX64 {
        bits: (target.base.bits & RegisterX64::INDEX_MASK) | size as u8,
      };
    } else if target.cat == CategoryX64::Mem {
      target.mem_size = size;
    }

    target
  }
}
