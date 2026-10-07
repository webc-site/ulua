//! DWARF x64 常量族（寄存器编号 + CFA 操作码；C++ DWARF2 编号/操作码表对偶）。
//! A64 LR/SP 与 x64 RA/RBP/RSP 已自单碎片迁回（r7-useline 票面 A：use 行白名单收口）。

pub const DW_REG_X64_RAX: i32 = 0;
pub const DW_REG_X64_RDX: i32 = 1;
pub const DW_REG_X64_RCX: i32 = 2;
pub const DW_REG_X64_RBX: i32 = 3;
pub const DW_REG_X64_RSI: i32 = 4;
pub const DW_REG_X64_RDI: i32 = 5;
pub const DW_REG_X64_RBP: i32 = 6;
pub const DW_REG_X64_RSP: i32 = 7;
pub const DW_REG_X64_RA: i32 = 16;
pub const DW_REG_A64_LR: i32 = 30;
pub const DW_REG_A64_SP: i32 = 31;

// DWARF CFA 指令操作码族（UnwindBuilderDwarf2.cpp #define 对偶；macros98 purge 前
// dw_cf_a_* 单源在此复位，值与 purge 前逐字相同）。
pub(crate) const DW_CFA_ADVANCE_LOC1: u8 = 0x02;
pub(crate) const DW_CFA_DEF_CFA: u8 = 0x0c;
pub(crate) const DW_CFA_DEF_CFA_OFFSET: u8 = 0x0e;
pub(crate) const DW_CFA_NOP: u8 = 0;
pub(crate) const DW_CFA_OFFSET: u8 = 0x80;
pub(crate) const DW_CFA_OFFSET_EXTENDED: u8 = 0x05;
