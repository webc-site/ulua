//! 来自 `CodeGen/src/EmitCommonA64.h` 的头文件级常量。
//! 注释逐字摘自上游对应行, 行号即 `cpp/CodeGen/src/EmitCommonA64.h` / `cpp/CodeGen/src/IrRegAllocA64.cpp` 中的行号。
//! 本模块是 A64 帧/spill 槽布局常量的**单一来源**：entry 帧大小、spill 寻址基址、
//! 分配器位图容量都从这里派生，禁止在函数体内再局部硬编码同名常量。

/// `inline constexpr unsigned kStashSlots = 9;  // stashed non-volatile registers` (EmitCommonA64.h:42)
pub const K_STASH_SLOTS: u32 = 9;

/// `inline constexpr unsigned kTempSlots = 1;   // 8 bytes of temporary space, such luxury!` (EmitCommonA64.h:43)
pub const K_TEMP_SLOTS: u32 = 1;

/// `inline constexpr unsigned kSpillSlots = 22; // slots for spilling temporary registers` (EmitCommonA64.h:44)
///
/// 上游为寄存器分配器预留的 8 字节 spill 槽数量：帧大小固定为
/// `inline constexpr unsigned kStackSize = (kStashSlots + kTempSlots + kSpillSlots) * 8;` (EmitCommonA64.h:46)
/// = (9 + 1 + 22) * 8 = 256 字节，分配器位图覆盖 K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS 槽：
/// `CODEGEN_ASSERT(kSpillSlots < 64);` / `freeSpillSlots = (1ull << kSpillSlots) - 1ull;` (IrRegAllocA64.cpp:147-148)。
pub const K_SPILL_SLOTS: u32 = 22;

/// `inline constexpr unsigned kStackSize = (kStashSlots + kTempSlots + kSpillSlots) * 8;` (EmitCommonA64.h:46)
/// = (9 + 1 + 22) * 8 = 256，entry 函数唯一一次 `sub sp, sp, kStackSize`（CodeGenA64.cpp:243）的尺寸。
pub const K_STACK_SIZE: u32 = (K_STASH_SLOTS + K_TEMP_SLOTS + K_SPILL_SLOTS) * 8;

/// spill 槽区在 entry 帧内的基址偏移：
/// `inline constexpr AddressA64 sSpillArea = mem(sp, (kStashSlots + kTempSlots) * 8);` (EmitCommonA64.h:48)
pub const S_SPILL_AREA: u32 = (K_STASH_SLOTS + K_TEMP_SLOTS) * 8;

/// 8 字节临时槽基址：`inline constexpr AddressA64 sTemporary = mem(sp, kStashSlots * 8);` (EmitCommonA64.h:49)
pub const S_TEMPORARY: u32 = K_STASH_SLOTS * 8;

// 编译期锚定帧不变量（对齐 cpp IrRegAllocA64.cpp:28 `CODEGEN_ASSERT(kStackSize <= 256)`）：
// 全部非 extra spill 槽的寻址 `mem(sp, S_SPILL_AREA + slot*8)` 的最高字节必须落在帧内。
const _: () = assert!(K_STACK_SIZE == (9 + 1 + 22) * 8);
const _: () = assert!(S_SPILL_AREA + K_SPILL_SLOTS * 8 <= K_STACK_SIZE);
const _: () = assert!(K_SPILL_SLOTS < 64);

/// ulua 扩展，**上游 `EmitCommonA64.h` 中不存在 `kExtraSpillSlots`**
/// （`grep -rn "ExtraSpill" cpp/CodeGen/{src,include}` 为空，A64/X64 两个后端都没有该概念）。
///
/// 位图在 K_SPILL_SLOTS 之外多发放本常量数量的槽（slot = 22..=23），这些槽不占 entry 帧，
/// 而是存放于 `global_State.ecbdata` 起始处，寻址 `(slot - K_SPILL_SLOTS) * 8`
/// （`methods/ir_reg_alloc_a_64_{spill,restore}_ir_reg_alloc_a_64.rs`，
/// 边界判定 `is_extra_spill_slot` 同样以 K_SPILL_SLOTS 为单一来源）。
pub const K_EXTRA_SPILL_SLOTS: u32 = 2;

// 分配器位图必须严格覆盖栈槽 + extra 槽（cpp IrRegAllocA64.cpp:147 同式，宽度并入 extra）。
const _: () = assert!(K_SPILL_SLOTS + K_EXTRA_SPILL_SLOTS < 64);
