use crate::{
  enums::abix_64::ABIX64, functions::get_non_vol_xmm_storage_size::get_non_vol_xmm_storage_size,
  records::emit_common_x_64::K_SPILL_SLOTS,
};

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
