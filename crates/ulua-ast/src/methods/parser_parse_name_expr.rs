use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_error::AstExprError,
    ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, node_handle::OptNode,
    parser::Parser,
  },
};

impl Parser {
  pub fn parse_name_expr(&mut self, context: &str) -> *mut AstExpr {
    let Some(name) = self.parse_name_opt(context) else {
      let location = self.lexer.current().location;
      let expressions = self.copy_initializer_list_t::<*mut AstExpr>(&[]);
      let message_index = (self.parse_errors.len() as u32).saturating_sub(1);

      return self.alloc_expr(AstExprError::new(location, expressions, message_index));
    };

    // 符号表槽位（cpp `if (value && *value)`）：`to_option` 把「键未登记」与「槽位为
    // nullptr」一并折成 None（落回全局分支），命中侧交出**自有**句柄——`local.*`
    // 的读取只借用本轮局部，与随后以 `&mut self` 报错/分配不重叠，arena 引用不再
    // 被铸成假的 `'static`，`Node::from_raw` 的现场折算也随之消失。
    if let Some(local) = self.local_map.find(&name.name).and_then(OptNode::to_option) {
      if local.function_depth < self.type_function_depth {
        return self.report_expr_error(
          self.lexer.current().location,
          AstArray::EMPTY,
          format_args!("Type function cannot reference outer local '{}'", local.name),
        );
      }

      let upvalue = local.function_depth != self.function_stack.len().saturating_sub(1);

      return self.alloc_expr(AstExprLocal::new(name.location, local, upvalue));
    }

    self.alloc_expr(AstExprGlobal::new(name.location, name.name))
  }
}
