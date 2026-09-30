//! arena AST 节点句柄重导出（复用 `ulua_ast::records::node_handle::Node`）。
//!
//! AST 节点由 `Reducer` 持有的 `Box<Allocator>` 页分配、随 `Reducer` 一起释放，
//! 天然满足「句柄在 Reducer 借用期内存活」。句柄本身只是**数据坐标**
//! （与 cpp 的 `AstStatBlock*` 同构，`Copy`）。节点解引用通过 `Node::get`/`Node::get_mut`。

pub use ulua_ast::records::node_handle::Node;
use ulua_ast::records::{ast_stat::AstStat, ast_stat_block::AstStatBlock};

/// 语句块句柄。
pub type Block = Node<AstStatBlock>;
/// 语句句柄。
pub type Stat = Node<AstStat>;
