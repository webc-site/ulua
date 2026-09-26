use alloc::vec::Vec;

use crate::{
  enums::brace_type::BraceType,
  records::{ast_name_table::AstNameTable, lexeme::Lexeme, location::Location},
};

#[derive(Debug, Clone)]
pub struct Lexer {
  /// cpp `Lexer` 的 `buffer`/`bufferSize` 成对入参的 Rust 形态：整段只读源缓冲，
  /// 长度即 `buffer.len()`，不再有独立可漂移的 size 参数（cpp 判界式
  /// `offset < bufferSize` 逐处折为 `offset < buffer.len()`，比较值不变）。
  ///
  /// 契约（唯一写点 [`Lexer::new`](crate::methods::lexer_lexer) 处兑现，与 cpp 非
  /// 拥有 `const char*` 同义）：源缓冲存活不短于 Lexer 及其产出的全部词素——
  /// cpp `Lexeme::data` 臂本就存 `&buffer[startOffset]`，寿命由解析会话保证；
  /// `&'static` 是该契约的可锻造形态（同 `functions/optional_node.rs::node_ref`
  /// 的「宿主活过使用期」先例），换取本 crate 读路径（peekch/切片分片）零
  /// `unsafe`。不引入新的空哨兵：空输入即空切片，null 指针形态不再存在。
  pub(crate) buffer: &'static [u8],

  pub(crate) offset: u32,

  pub(crate) line: u32,
  pub(crate) line_offset: u32,

  pub(crate) lexeme: Lexeme,

  pub(crate) prev_location: Location,

  pub(crate) names: *mut AstNameTable,

  pub(crate) skip_comments: bool,
  pub(crate) read_names: bool,

  pub(crate) brace_stack: Vec<BraceType>,
}

// Safety: Lexer 的裸指针字段仅剩 `names`（指向本身声明 Send+Sync 的 AstNameTable）
// 与 `lexeme` 内 `LexemeData::data` 指针字段（只读源缓冲/名表驻留串的地址值，二者均比
// Lexer 长寿，见上契约）；其余全是切片引用/Location/计数器/Vec 等 plain data。跨
// 线程 move 只转移这两个引用目标，而它们在各自文档契约下本可移动，故 Send 成立。
unsafe impl Send for Lexer {}
// Safety: `&Lexer` 只读取所借的输入切片（`&'static [u8]` 天然 Sync，契约要求源
// 缓冲活过解析会话），并经 `*mut AstNameTable` 触达名字表；后者由 AstNameTable 的
// Sync 覆盖共享访问。单线程串行词法下对同一表的访问仍遵循 allocator 的单次访问
// 约束，故 Sync 成立。
unsafe impl Sync for Lexer {}
