/// GC 标记字节中的 FIXED 位（固定对象，不可回收）
pub const FIXEDBIT: i32 = 3;

// WHITE0BIT/WHITE1BIT 统一以 `whitebits` 为准，此处仅转发保持旧路径可用
pub use crate::macros::whitebits::{WHITE0BIT, WHITE1BIT, WHITEBITS};
