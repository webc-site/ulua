use core::{ffi::c_void, ptr::null};

use ulua_ast::records::{
  ast_expr_constant_string::AstExprConstantString, ast_expr_table::AstExprTable, ast_node::AstNode,
};

use crate::records::{
  data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, def::Def,
  symbol::Symbol,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_table(&mut self, t: *mut AstExprTable) -> DataFlowResult {
    unsafe {
      let def_arena = self.def_arena;
      let table_cell = (*def_arena).fresh_cell(Symbol::default(), (*t).base.base.location, false);
      let scope = self.current_scope();
      // C++: currentScope()->props[tableCell] = {};
      *(*scope).props.get_or_insert(table_cell) = Default::default();

      for item in (*t).items.as_slice() {
        let result = self.visit_expr_ast_expr(item.value);
        if !item.key.is_null() {
          self.visit_expr_ast_expr(item.key);
          let key_node = item.key as *mut AstNode;
          if (*key_node).is::<AstExprConstantString>() {
            let string = &*(item.key as *mut AstExprConstantString);
            let key_str = String::from_utf8_lossy(string.value.as_bytes()).into_owned();
            // C++: currentScope()->props[tableCell][string->value.data] = result.def;
            let props = (*scope).props.get_or_insert(table_cell);
            let def = result.def as *const Def;
            props.insert(key_str, def);
          }
        }
      }

      // C++: return {tableCell, nullptr};
      DataFlowResult {
        def: table_cell as *const c_void,
        parent: null(),
      }
    }
  }
}
