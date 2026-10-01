//! 来自 `CodeGen/src/EmitCommonX64.h` 的头文件级常量。
//! 注释逐字摘自上游对应行，行号即 `cpp/CodeGen/src/EmitCommonX64.h` 中的行号。

use crate::records::register_x_64::RegisterX64;

/// `inline constexpr uint32_t kFunctionAlignment = 32;` (EmitCommonX64.h:36)
pub const K_FUNCTION_ALIGNMENT: u32 = 32;
/// `inline constexpr RegisterX64 rState = r15;  // lua_State* L` (EmitCommonX64.h:39)
pub const R_STATE: RegisterX64 = RegisterX64::R15;
/// `inline constexpr RegisterX64 rBase = r14;  // StkId base` (EmitCommonX64.h:40)
pub const R_BASE: RegisterX64 = RegisterX64::R14;
/// `inline constexpr RegisterX64 rNativeContext = r13;  // NativeContext* context` (EmitCommonX64.h:41)
pub const R_NATIVE_CONTEXT: RegisterX64 = RegisterX64::R13;
/// `inline constexpr RegisterX64 rConstants = r12;  // TValue* k` (EmitCommonX64.h:42)
pub const R_CONSTANTS: RegisterX64 = RegisterX64::R12;
/// `inline constexpr unsigned kSpillSlots = 23;` (EmitCommonX64.h:45)
/// 栈帧中为寄存器分配器预留的 8 字节 spill 槽数量, 超出即放弃编译回退解释器。
/// 帧大小（`functions::get_full_stack_size`）与回退阈值（`IrLoweringX64::hasError`）共用本常量。
pub const K_SPILL_SLOTS: u32 = 23;
/// `constexpr uint8_t kNoStackSlot = 0xff;` (IrRegAllocX64.h:24)
/// 注意：上游定义在 `IrRegAllocX64.h` 而非本头文件；此处是提升到 `u32` 的比较用形式，
/// 值的单一来源是 `IrSpillX64::K_NO_STACK_SLOT`。
pub const K_NO_STACK_SLOT: u32 = 0xff;
