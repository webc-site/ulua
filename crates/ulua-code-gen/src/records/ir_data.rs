//! Header-level constants from `CodeGen/include/Luau/IrData.h`.

/// `inline constexpr uint32_t kInvalidInstIdx = ~0u;` (IrData.h:1234)
pub const K_INVALID_INST_IDX: u32 = u32::MAX;

/// `constexpr uint8_t kUnknownTag = 0xff;`（IrData.h）：寄存器 tag 未知的哨兵值。
pub const K_UNKNOWN_TAG: u8 = 0xff;

/// cpp `OptimizeConstProp.cpp` upvalueMap 的空槽占位 `~0u8`：与 `K_UNKNOWN_TAG` 同值但
/// 语义无关（这里是「不存在的 upvalue 编号」，那里是「未知 tag」）。
pub const K_INVALID_UPVALUE: u8 = u8::MAX;
