use alloc::string::String;

use crate::{
  records::{ast_node::AstNode, printer::Printer, string_writer::StringWriter},
  type_aliases::cst_node_map::CstNodeMap,
};

/// Formats an AST node to a pretty-printed string.
///
/// 入参为 `&AstNode`：引用即「存活且只读」证明，本函数无前置条件；printer 全程
/// 只写 Writer、不回写节点，故共享借用足够（下转一律走 `as_*_const` 只读形态）。
pub fn to_string_ast_node(node: &AstNode) -> String {
  let mut writer = StringWriter {
    ss: Vec::new(),
    pos: node.location.begin,
    last_char: '\0',
  };

  let empty_cst_node_map = CstNodeMap::default();
  let mut printer = Printer::new(&mut writer, &empty_cst_node_map);
  printer.write_types = true;

  if let Some(stat_node) = node.as_stat_const() {
    // 打印器只写 Writer，节点全程共享借用（不再从共享借用造 &mut）。
    printer.visualize_ast_stat(stat_node);
  } else if let Some(expr_node) = node.as_expr_const() {
    printer.visualize_ast_expr(expr_node);
  } else {
    printer.visualize_type_annotation(node.as_type_const());
  }

  // 接管 writer 内部缓冲，省一次整串拷贝。fixup 后的字符串值可含
  // 任意字节（`"\xff"` 非法 UTF-8），此 String 出口无法表达非 UTF-8 字节，
  // 非法序列经 from_utf8_lossy 替换为 U+FFFD（与 cpp `std::string` 返回的
  // 语义差异）；逐字节语义走 `StringWriter::take_bytes`。
  String::from_utf8_lossy(&writer.take_bytes()).into_owned()
}
