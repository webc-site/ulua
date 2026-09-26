//! 来自 `CodeGen/include/Luau/IrData.h` 的头文件级常量。

use core::mem::size_of;

/// `inline constexpr uint32_t kInvalidInstIdx = ~0u;` (IrData.h:1234)
pub const K_INVALID_INST_IDX: u32 = u32::MAX;

/// `constexpr uint8_t kUnknownTag = 0xff;`（IrData.h）：寄存器 tag 未知的哨兵值。
pub const K_UNKNOWN_TAG: u8 = 0xff;

/// cpp `OptimizeConstProp.cpp` upvalueMap 的空槽占位 `~0u8`：与 `K_UNKNOWN_TAG` 同值但
/// 语义无关（这里是「不存在的 upvalue 编号」，那里是「未知 tag」）。
pub const K_INVALID_UPVALUE: u8 = u8::MAX;

/// native codegen 目标的数据指针宽度：cpp 布局镜像里以 `sizeof(Proto*)` /
/// `sizeof(TString*)` 出现（同为地址宽度，所有受支持目标恒等于宿主 `usize` 宽度）。
/// 以常量替代 `size_of::<*mut Proto>()` 等裸指针字面写法，行为不变且收敛指针面。
pub const K_NATIVE_PTR_SIZE: usize = size_of::<usize>();
