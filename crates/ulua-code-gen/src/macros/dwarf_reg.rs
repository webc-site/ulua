//! DWARF x64 常量族（寄存器编号 + CFA 操作码；C++ DWARF2 编号/操作码表对偶）。
//! RBP/RSP/RA 与 A64 LR/SP 因黑名单消费方（records/unwind_builder_dwarf_2.rs）
//! 的模块路径锁定，保留在原独立文件中。

pub const DW_REG_X64_RAX: i32 = 0;
pub const DW_REG_X64_RDX: i32 = 1;
pub const DW_REG_X64_RCX: i32 = 2;
pub const DW_REG_X64_RBX: i32 = 3;
pub const DW_REG_X64_RSI: i32 = 4;
pub const DW_REG_X64_RDI: i32 = 5;

// DWARF CFA 指令操作码族（UnwindBuilderDwarf2.cpp #define 对偶；macros98 purge 前
// dw_cf_a_* 单源在此复位，值与 purge 前逐字相同）。
pub(crate) const DW_CFA_ADVANCE_LOC1: u8 = 0x02;
pub(crate) const DW_CFA_DEF_CFA: u8 = 0x0c;
pub(crate) const DW_CFA_DEF_CFA_OFFSET: u8 = 0x0e;
pub(crate) const DW_CFA_NOP: u8 = 0;
pub(crate) const DW_CFA_OFFSET: u8 = 0x80;
pub(crate) const DW_CFA_OFFSET_EXTENDED: u8 = 0x05;
