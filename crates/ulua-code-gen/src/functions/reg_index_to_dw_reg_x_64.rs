use crate::macros::{
  dw_reg_x_64_rax::DW_REG_X64_RAX, dw_reg_x_64_rbp::DW_REG_X64_RBP,
  dw_reg_x_64_rbx::DW_REG_X64_RBX, dw_reg_x_64_rcx::DW_REG_X64_RCX,
  dw_reg_x_64_rdi::DW_REG_X64_RDI, dw_reg_x_64_rdx::DW_REG_X64_RDX,
  dw_reg_x_64_rsi::DW_REG_X64_RSI, dw_reg_x_64_rsp::DW_REG_X64_RSP,
};

pub fn reg_index_to_dw_reg_x_64(index: u8) -> i32 {
  match index {
    0 => DW_REG_X64_RAX,
    1 => DW_REG_X64_RCX,
    2 => DW_REG_X64_RDX,
    3 => DW_REG_X64_RBX,
    4 => DW_REG_X64_RSP,
    5 => DW_REG_X64_RBP,
    6 => DW_REG_X64_RSI,
    7 => DW_REG_X64_RDI,
    8..=15 => index as i32,
    // 编译器内部不变量：调用方仅传 Register/X64 寄存器的 0..15 下标（与 cpp UnwindBuilder
    // 的 DW_REG_X64 表覆盖域一致），越界即寄存器编号被破坏，属不可达分支的显式证成。
    _ => panic!("无效 x64 寄存器下标 {index}：合法域恒为 0..15"),
  }
}
