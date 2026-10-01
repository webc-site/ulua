use crate::{
  functions::optional_node::slot_opt,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_error::AstExprError,
    ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, node_handle::Node,
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

    let value = self.local_map.find(&name.name);

    if let Some(&local) = value {
      // 符号表值是 arena 中存活的 `AstLocal` 节点或 null：slot_opt 把 null 槽
      // 折叠为 None（落回全局分支，与旧 filter 早退等价），Some 侧共享借用读取
      // `function_depth`/`name` 字段（local_map 全会话持有该节点、地址不移动）。
      if let Some(local_node) = slot_opt(local) {
        if local_node.function_depth < self.type_function_depth {
          return self.report_expr_error(
            self.lexer.current().location,
            AstArray::EMPTY,
            format_args!(
              "Type function cannot reference outer local '{}'",
              local_node.name
            ),
          );
        }

        let upvalue = local_node.function_depth != self.function_stack.len().saturating_sub(1);

        // local 出自 local_map 命中分支（cpp 同款 `if (value && *value)` 守卫），恒非空。
        return self.alloc_expr(AstExprLocal::new(
          name.location,
          Node::from_raw(local),
          upvalue,
        ));
      }
    }

    self.alloc_expr(AstExprGlobal::new(name.location, name.name))
  }
}
