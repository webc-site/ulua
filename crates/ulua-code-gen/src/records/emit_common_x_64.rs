//! Header-level constants from `CodeGen/src/EmitCommonX64.h`.

/// `inline constexpr uint32_t kFunctionAlignment = 32;`
pub const K_FUNCTION_ALIGNMENT: u32 = 32;
/// `inline constexpr unsigned kSpillSlots = 13;`
pub const K_SPILL_SLOTS: u32 = 13;
/// `inline constexpr unsigned kExtraSpillSlots = 64;`
pub const K_EXTRA_SPILL_SLOTS: u32 = 64;
/// `kNoStackSlot` (IrData.h): `0xff`, promoted to unsigned at comparison sites.
pub const K_NO_STACK_SLOT: u32 = 0xff;
