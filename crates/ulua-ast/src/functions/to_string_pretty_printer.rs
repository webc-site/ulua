use alloc::string::String;
use core::ptr::null_mut;

use crate::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, printer::Printer,
    string_writer::StringWriter,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

/// Formats an AST node to a pretty-printed string.
///
/// # Safety
/// `node` must point to a live AST node.
pub unsafe fn to_string_ast_node(node: *mut AstNode) -> String {
  let node_ref = unsafe { &*node };

  let mut writer = StringWriter {
    ss: Vec::new(),
    pos: node_ref.location.begin,
    last_char: '\0',
  };

  let mut printer = Printer::new(&mut writer, CstNodeMap::new(null_mut()));
  printer.write_types = true;

  let stat_node = node_ref.as_stat_const();
  if !stat_node.is_null() {
    // 打印器只写 Writer，节点全程共享借用（不再从共享借用造 &mut）
    printer.visualize_ast_stat(unsafe { &*stat_node });
  } else {
    let expr_node = node_ref.as_expr_const();
    if !expr_node.is_null() {
      printer.visualize_ast_expr(unsafe { &*expr_node });
    } else {
      printer.visualize_type_annotation(unsafe { &*node_ref.as_type() });
    }
  }

  // 接管 writer 内部缓冲，省一次整串拷贝。fixup 后的字符串值可含
  // 任意字节（`"\xff"` 非法 UTF-8），此 String 出口无法表达非 UTF-8 字节，
  // 非法序列经 from_utf8_lossy 替换为 U+FFFD（与 cpp `std::string` 返回的
  // 语义差异）；逐字节语义走 `StringWriter::take_bytes`。
  String::from_utf8_lossy(&writer.take_bytes()).into_owned()
}
