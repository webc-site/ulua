//! 本目录只保留**无法并回所属类型定义**的自由方法文件。
//!
//! - `dense_hash_table_find`：`DenseHashTable` 的按键查找以自由函数形态被
//!   `ulua-analysis` 按模块路径引用（`ulua_common::methods::dense_hash_table_find::…`），
//!   并回 `records/dense_hash_table.rs` 会破坏该跨 crate import，故保留。
//!
//! 其余 11 枚单方法碎片（`f_value_*`、`global_context_*`、`scope_*`、`thread_context_*`、
//! `variant_get_*`）已全部并回 `records/<type>.rs` 的 impl 块：它们只是所属类型的一个
//! 方法、跨 crate 侧零 `methods::` 路径消费者（impl 方法经类型解析，不需要模块路径），
//! 且文件名是 `thread_context_thread_context_time_trace.rs` 这类机械退化名，不构成独立
//! 概念。`VecDeque` 的 11 枚同类碎片此前已并回 `records/vec_deque.rs`。
pub mod dense_hash_table_find;
