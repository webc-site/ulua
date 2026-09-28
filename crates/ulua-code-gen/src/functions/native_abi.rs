use core::mem::size_of;

use crate::{
  enums::abix_64::ABIX64,
  records::{
    emit_common_x_64::K_SPILL_SLOTS, native_proto_exec_data_header::NativeProtoExecDataHeader,
  },
};

pub fn get_current_abi() -> ABIX64 {
  #[cfg(target_os = "windows")]
  {
    ABIX64::Windows
  }
  #[cfg(not(target_os = "windows"))]
  {
    ABIX64::SYSTEM_V
  }
}

pub const K_SYSTEM_VUSABLE_XMM_REGS: u8 = 16;
pub const K_WINDOWS_USABLE_XMM_REGS: u8 = 10;

#[inline]
pub fn get_xmm_register_count(abi: ABIX64) -> u8 {
  if abi == ABIX64::SYSTEM_V {
    K_SYSTEM_VUSABLE_XMM_REGS
  } else {
    K_WINDOWS_USABLE_XMM_REGS
  }
}

const K_WINDOWS_FIRST_NON_VOL_XMM_REG: u8 = 6;

pub fn get_non_vol_xmm_storage_size(abi: ABIX64, xmm_reg_count: u8) -> u32 {
  if abi == ABIX64::SYSTEM_V {
    return 0;
  }

  // 前 6 个是 volatile
  if xmm_reg_count <= K_WINDOWS_FIRST_NON_VOL_XMM_REG {
    return 0;
  }

  assert!(xmm_reg_count <= 16);
  u32::from(xmm_reg_count - K_WINDOWS_FIRST_NON_VOL_XMM_REG) * 16
}

pub const K_STACK_ALIGN: u32 = 8;
pub const K_STACK_LOCAL_STORAGE: u32 = 8 * 3;
// 栈帧为寄存器分配器预留 8*kSpillSlots 字节, 与 has_error 判据共用同一常量
pub const K_STACK_SPILL_STORAGE: u32 = 8 * K_SPILL_SLOTS;
pub const K_STACK_EXTRA_ARGUMENT_STORAGE: u32 = 2 * 8;
pub const K_STACK_REG_HOME_STORAGE: u32 = 4 * 8;
pub const K_STACK_OFFSET_TO_LOCALS: u32 = K_STACK_EXTRA_ARGUMENT_STORAGE + K_STACK_REG_HOME_STORAGE;
pub const K_STACK_OFFSET_TO_SPILL_SLOTS: u32 = K_STACK_OFFSET_TO_LOCALS + K_STACK_LOCAL_STORAGE;

#[inline]
pub fn get_full_stack_size(abi: ABIX64, xmm_reg_count: u8) -> u32 {
  K_STACK_OFFSET_TO_SPILL_SLOTS
    + K_STACK_SPILL_STORAGE
    + get_non_vol_xmm_storage_size(abi, xmm_reg_count)
    + K_STACK_ALIGN
}

#[inline]
pub fn compute_native_exec_data_size(
  bytecode_instruction_count: u32,
  extra_data_count: u32,
) -> usize {
  let header_size = size_of::<NativeProtoExecDataHeader>();
  let bytecode_size = (bytecode_instruction_count as usize) * size_of::<u32>();
  let extra_data_size = (extra_data_count as usize) * size_of::<u32>();
  header_size + bytecode_size + extra_data_size
}
