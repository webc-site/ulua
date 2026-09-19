//! Header-level constants from `CodeGen/include/Luau/IrData.h`.

/// `inline constexpr uint32_t kInvalidInstIdx = ~0u;` (IrData.h:1234)
pub const K_INVALID_INST_IDX: u32 = !0u32;

/// `constexpr uint8_t kUnknownTag = 0xff;`（IrData.h）：寄存器 tag 未知的哨兵值。
pub const K_UNKNOWN_TAG: u8 = 0xff;
