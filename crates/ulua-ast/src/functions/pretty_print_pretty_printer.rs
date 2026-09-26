use alloc::{string::String, vec::Vec};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::quote_style_cst::QuoteStyle,
  functions::pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block_cst_node_map,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
    location::Location, parse_options::ParseOptions, parser::Parser, position::Position,
    pretty_print_result::PrettyPrintResult, printer::Printer, string_writer::StringWriter,
    writer::Writer,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

pub fn pretty_print_ast_stat_block_cst_node_map(
  block: &mut AstStatBlock,
  cst_node_map: &CstNodeMap,
) -> String {
  pretty_print_to_string(block, cst_node_map, false)
}

/// cpp `prettyPrint` 的逐字节出口：原样返回 `StringWriter` 缓冲。
///
/// `String` 出口经 `from_utf8_lossy`，词法器 fixup 后的任意字节（`"\xff"`
/// 不是合法 UTF-8）会被替换成 U+FFFD；要 cpp `std::string` 的逐字节语义
/// 只能走这里。
pub fn pretty_print_ast_stat_block_cst_node_map_bytes(
  block: &mut AstStatBlock,
  cst_node_map: &CstNodeMap,
) -> Vec<u8> {
  pretty_print_impl(block, cst_node_map, false)
}

/// 两个 String 入口（pretty_print / pretty_print_with_types）共用的脚手架。
pub(crate) fn pretty_print_to_string(
  block: &mut AstStatBlock,
  cst_node_map: &CstNodeMap,
  write_types: bool,
) -> String {
  // 直接接管 writer 内部缓冲，省一次整串拷贝。fixup 后的字符串值可含
  // 任意字节（`"\xff"` 非法 UTF-8），此 String 出口无法表达非 UTF-8 字节，
  // 非法序列经 from_utf8_lossy 替换为 U+FFFD（与 cpp `std::string` 返回的
  // 语义差异）；逐字节语义走 `pretty_print_ast_stat_block_cst_node_map_bytes`。
  let bytes = pretty_print_impl(block, cst_node_map, write_types);
  String::from_utf8_lossy(&bytes).into_owned()
}

/// Printer 泛型于 `W: Writer`：此处静态单态化到 StringWriter，无 dyn 开销。
fn pretty_print_impl(
  block: &mut AstStatBlock,
  cst_node_map: &CstNodeMap,
  write_types: bool,
) -> Vec<u8> {
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

  writer.take_bytes()
}

/// `Writer` 的 12 个方法全部只是把实参原样转发到 `StringWriter` 的同名固有
/// 方法（cpp 侧 `StringWriter` 逐条 `override` 纯虚函数，无适配层）。手写 12
/// 条重复样板既冗长又会与固有方法签名漂移，宏一次生成；固有方法优先于 trait
/// 方法解析，故 `self.$name(..)` 落在固有实现上、不会自递归。
macro_rules! forward_writer_methods {
  ($(fn $name:ident(&mut self $(, $arg:ident : $ty:ty)*);)*) => {
    impl Writer for StringWriter {
      $(
        fn $name(&mut self $(, $arg: $ty)*) {
          self.$name($($arg),*)
        }
      )*
    }
  };
}

forward_writer_methods! {
  fn advance(&mut self, pos: &Position);
  fn newline(&mut self);
  fn space(&mut self);
  fn maybe_space(&mut self, new_pos: &Position, reserve: i32);
  fn write(&mut self, s: &[u8]);
  fn write_multiline(&mut self, s: &[u8]);
  fn identifier(&mut self, name: &[u8]);
  fn keyword(&mut self, s: &str);
  fn symbol(&mut self, s: &str);
  fn literal(&mut self, s: &[u8]);
  fn string(&mut self, s: &[u8]);
  fn source_string(&mut self, s: &[u8], quote_style: QuoteStyle, block_depth: u32);
}

pub fn pretty_print_ast_stat_block(block: &mut AstStatBlock) -> String {
  pretty_print_ast_stat_block_cst_node_map(block, &CstNodeMap::default())
}

pub fn pretty_print_string_view_parse_options_bool_bool(
  source: &str,
  mut options: ParseOptions,
  with_types: bool,
  ignore_parse_errors: bool,
) -> PrettyPrintResult {
  options.store_cst_data = true;

  // Box 钉堆：AstNameTable/Parser 内部存 `*mut Allocator`（捕获宿主地址），
  // 宿主一旦移动即悬垂（同 ulua-compiler tests.rs string_table! 先例）。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_result = Parser::parse(source, &mut names, &mut allocator, options);

  let has_errors = !parse_result.errors.is_empty();

  if has_errors && !ignore_parse_errors {
    let error = &parse_result.errors[0];
    return PrettyPrintResult {
      code: String::new(),
      error_location: *error.get_location(),
      parse_error: error.what().to_string(),
    };
  }

  LUAU_ASSERT!(!parse_result.root.is_null());
  if parse_result.root.is_null() {
    return PrettyPrintResult {
      code: String::new(),
      error_location: Location::default(),
      parse_error: String::from("Internal error: Parser yielded empty parse tree"),
    };
  }

  let root = unsafe {
    // Safety: 上方已对 parse_result.root 判空返回；root 是本次 parse 在 arena 构造的存活 AstStatBlock，parse_result 被本函数独占消费，&mut 重建无其他活跃借用（单线程）。
    &mut *parse_result.root
  };
  if with_types {
    PrettyPrintResult {
      code: pretty_print_with_types_ast_stat_block_cst_node_map(root, &parse_result.cst_node_map),
      error_location: Location::default(),
      parse_error: String::new(),
    }
  } else {
    PrettyPrintResult {
      code: pretty_print_ast_stat_block_cst_node_map(root, &parse_result.cst_node_map),
      error_location: Location::default(),
      parse_error: String::new(),
    }
  }
}
