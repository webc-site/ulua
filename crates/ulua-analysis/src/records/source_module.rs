//! Source: `Analysis/include/Luau/Module.h`

// Module.h:36 — hand-ported; field set matches the C++ struct.
use alloc::{string::String, sync::Arc, vec::Vec};

use ulua_ast::{
  enums::mode,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
    comment::Comment, hot_comment::HotComment, parse_error::ParseError,
  },
};

use crate::{enums::type_file_resolver::Type, type_aliases::module_name_type::ModuleName};
#[derive(Debug, Clone)]
pub struct SourceModule {
  pub name: ModuleName, // Module identifier or a filename
  pub human_readable_name: String,
  pub r#type: Type,
  pub environment_name: Option<String>,
  pub cyclic: bool,
  pub allocator: Arc<Allocator>,
  pub names: Arc<AstNameTable>,
  pub parse_errors: Vec<ParseError>,
  pub root: *mut AstStatBlock,
  pub mode: Option<mode::Mode>,
  pub hotcomments: Vec<HotComment>,
  pub comment_locations: Vec<Comment>,
}

/// # Safety
///
/// `root: *mut AstStatBlock` 借用自 `allocator: Arc<Allocator>` 拥有的 AST，令
/// 自动 Send 失效。该指针不被本结构解引用或释放，且 AST 由 `Arc<Allocator>`
/// /`Arc<AstNameTable>` 保有并跨线程存活；满足此所有权时转移即可靠。
// Safety: 见上方文档——`root` 只是 arena 内 AST 结点的身份句柄（本结构不解引用/释放它），
// AST 内存由同为字段成员的 `Arc<Allocator>` 保活，转移 SourceModule 即一并转移该 Arc 的
// 引用计数所有权，故跨线程移交后指针依旧有效。
unsafe impl Send for SourceModule {}
/// # Safety
///
/// 同上：`root` 仅身份借用，`allocator`/`names` 为 `Arc`（其内部 `Allocator` 已
/// `Send + Sync`），共享 `&SourceModule` 只读遍历 AST 不产生数据竞争。
// Safety: 见上方文档——共享 `&SourceModule` 时 `root` 仅被只读遍历（AST 在 parse 完成后
// 冻结），`Allocator`/`AstNameTable` 本身已实现 `Sync`，故并发只读不构成数据竞争。
unsafe impl Sync for SourceModule {}
