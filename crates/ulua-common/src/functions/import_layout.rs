//! `LOP_GETIMPORT` 线格式（aux / import id 字，二者同构打包）布局：高 2 位为组件数，
//! 其余从高到低每 10 位一个组件的常量索引，组件 `k` 位于 `[30-10(k+1), 30-10k)`。
//! 对应 cpp BytecodeBuilder.h 的 `kImportCountShift` / `kImportComponentBits` /
//! `kImportComponentMask`。编码侧（ulua-bytecode 的 `get_import_id*`、`emit_instruction`）
//! 与解码/执行侧（`decode_import_aux`、`luaV_getimport`）共用，布局改动只需改这一处。

/// 高 2 位组件数计数字段的移位量。
pub const K_IMPORT_COUNT_SHIFT: u32 = 30;
/// 单个导入组件常量索引占用的位数。
pub const K_IMPORT_COMPONENT_BITS: u32 = 10;
/// 单个导入组件的 10 位掩码。
pub const K_IMPORT_COMPONENT_MASK: u32 = 0x3ff;

/// 组件 `k`（0 起）在 aux/import id 字中的移位量：`30-10(k+1)`。
pub const fn import_component_shift(k: u32) -> u32 {
  K_IMPORT_COUNT_SHIFT - (k + 1) * K_IMPORT_COMPONENT_BITS
}

/// 从 import id / aux 字中取出组件 `k` 的 10 位常量索引。
pub const fn import_component(id: u32, k: u32) -> u32 {
  (id >> import_component_shift(k)) & K_IMPORT_COMPONENT_MASK
}
