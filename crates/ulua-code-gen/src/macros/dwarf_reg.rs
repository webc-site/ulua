//! DWARF x64 寄存器编号常量族（C++ DWARF2 编号表对偶）。
//! RBP/RSP/RA 与 A64 LR/SP 因黑名单消费方（records/unwind_builder_dwarf_2.rs）
//! 的模块路径锁定，保留在原独立文件中。

pub const DW_REG_X64_RAX: i32 = 0;
pub const DW_REG_X64_RDX: i32 = 1;
pub const DW_REG_X64_RCX: i32 = 2;
pub const DW_REG_X64_RBX: i32 = 3;
pub const DW_REG_X64_RSI: i32 = 4;
pub const DW_REG_X64_RDI: i32 = 5;
