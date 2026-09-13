use alloc::string::String;
use core::ptr::null_mut;

use crate::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, printer::Printer,
    string_writer::StringWriter, writer::Writer,
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
    ss: String::new(),
    pos: node_ref.location.begin,
    last_char: '\0',
  };

  let mut printer = Printer::new(&mut writer as &mut dyn Writer, CstNodeMap::new(null_mut()));
  printer.write_types = true;

  let stat_node = node_ref.as_stat_const();
  if !stat_node.is_null() {
    let stat_node_mut = unsafe { &mut *(stat_node as *mut AstStat) };
    printer.visualize_ast_stat(stat_node_mut);
  } else {
    let expr_node = node_ref.as_expr_const();
    if !expr_node.is_null() {
      let expr_node_mut = unsafe { &mut *(expr_node as *mut AstExpr) };
      printer.visualize_ast_expr(expr_node_mut);
    } else {
      let type_node = unsafe { &mut *node_ref.as_type() };
      printer.visualize_type_annotation(type_node);
    }
  }

  writer.str().clone()
}
