use alloc::{string::String, vec::Vec};

use crate::{
  enums::quote_style_cst::QuoteStyle,
  records::{
    ast_stat_block::AstStatBlock, position::Position, printer::Printer,
    string_writer::StringWriter, writer::Writer,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

pub fn pretty_print_ast_stat_block_cst_node_map(
  block: &mut AstStatBlock,
  cst_node_map: CstNodeMap,
) -> String {
  pretty_print_impl(block, cst_node_map, false)
}

/// 两个入口（pretty_print / pretty_print_with_types）共用的脚手架。
/// Printer 泛型于 `W: Writer`：此处静态单态化到 StringWriter，无 dyn 开销。
pub(crate) fn pretty_print_impl(
  block: &mut AstStatBlock,
  cst_node_map: CstNodeMap,
  write_types: bool,
) -> String {
  let mut writer = StringWriter {
    ss: Vec::new(),
    pos: Position::new(0, 0),
    last_char: '\0',
  };

  {
    let mut printer = Printer::new(&mut writer, cst_node_map);
    printer.write_types = write_types;
    printer.visualize_block_ast_stat_block(&*block);
  }

  // 直接接管 writer 内部缓冲，省一次整串拷贝。fixup 后的字符串值可含
  // 任意字节（`"\xff"` 非法 UTF-8），此 String 出口无法表达非 UTF-8 字节，
  // 非法序列经 from_utf8_lossy 替换为 U+FFFD（与 cpp `std::string` 返回的
  // 语义差异）；逐字节语义走 `StringWriter::take_bytes`。
  String::from_utf8_lossy(&writer.take_bytes()).into_owned()
}

impl Writer for StringWriter {
  fn advance(&mut self, new_pos: &Position) {
    self.advance(new_pos);
  }

  fn maybe_space(&mut self, new_pos: &Position, reserve: i32) {
    self.maybe_space(new_pos, reserve);
  }

  fn newline(&mut self) {
    self.newline();
  }

  fn space(&mut self) {
    self.space();
  }

  fn write_multiline(&mut self, s: &[u8]) {
    self.write_multiline(s);
  }

  fn write(&mut self, s: &[u8]) {
    self.write(s);
  }

  fn identifier(&mut self, s: &[u8]) {
    self.identifier(s);
  }

  fn keyword(&mut self, s: &str) {
    self.keyword(s);
  }

  fn symbol(&mut self, s: &str) {
    self.symbol(s);
  }

  fn literal(&mut self, s: &[u8]) {
    self.literal(s);
  }

  fn string(&mut self, s: &[u8]) {
    self.string(s);
  }

  fn source_string(&mut self, s: &[u8], quote_style: QuoteStyle, block_depth: u32) {
    self.source_string(s, quote_style, block_depth);
  }
}

#[cfg(test)]
mod tests {
  use alloc::{boxed::Box, vec::Vec};
  use core::ptr::null_mut;

  use super::*;
  use crate::{
    records::{
      allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
      parser::Parser,
    },
    type_aliases::cst_node_map::CstNodeMap,
  };

  /// 回归：`"\xff\x02"` 经词法器 fixup 产生非法 UTF-8 字节序列。pretty print
  /// 必须逐字节保留（cpp `std::string_view` 语义）：`string()` 无单引号时选
  /// `'` 引号，`0xFF` 可打印直接透传，`0x02` 控制字符走 `%03u` 十进制转义
  /// `\002`。对应 cpp PrettyPrinter.cpp StringWriter::string + Luau::escape。
  #[test]
  fn pretty_print_preserves_arbitrary_bytes_in_string_value() {
    // Box 钉堆：AstNameTable/Parser 内部存 `*mut Allocator`（捕获宿主地址），
    // 宿主移动即悬垂（同 pretty_print_string_view_parse_options_bool_bool 先例）。
    let mut allocator = Box::new(Allocator::new());
    let mut names = AstNameTable::new(&mut allocator);
    // 源码文本 `print("\xff\x02")`：`\xff`/`\x02` 是 Lua 转义序列（源 &str
    // 本身合法 UTF-8），lexer fixup_quoted_bytes 展开后 value = [0xFF, 0x02]。
    let source = "print(\"\\xff\\x02\")";
    let parse_result = Parser::parse(
      source,
      &mut names,
      &mut allocator,
      // store_cst_data = false：无 CST 时走 StringWriter::string 路径。
      ParseOptions::default(),
    );
    assert!(
      parse_result.errors.is_empty(),
      "parse failed: {:?}",
      parse_result.errors.first().map(|e| e.what())
    );

    let mut writer = StringWriter {
      ss: Vec::new(),
      pos: Position::new(0, 0),
      last_char: '\0',
    };
    {
      let mut printer = Printer::new(&mut writer, CstNodeMap::new(null_mut()));
      // SAFETY: parse 成功后 root 指向 arena 中存活的 AstStatBlock；打印器只写
      // Writer，节点全程共享借用
      let root = unsafe { &*parse_result.root };
      printer.visualize_block_ast_stat_block(root);
    }

    // 逐字节断言（cpp StringWriter::string 语义推演）：
    // - 引号：value 不含单引号 → 默认 `'`（仅含 `'` 时 cpp 才换 `"`）；
    // - 0xFF：`>= ' '` 且非特殊符号 → 原样透传（非法 UTF-8 不替换、不丢字节）；
    // - 0x02：控制字符 → `\` + `%03u` 十进制 `\002`；
    // - 尾部 3 空格：`advance` 按源码列位补齐列差（cpp 同款 `std::string(col, ' ')`）。
    let bytes: Vec<u8> = writer.take_bytes();
    assert_eq!(bytes, b"print('\xFF\\002')   ".as_slice());
  }
}
