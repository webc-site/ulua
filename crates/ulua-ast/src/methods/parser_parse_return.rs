use ulua_common::fflag::LuauExportValueSyntax;

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_stat::AstStat, ast_stat_return::AstStatReturn, cst_stat_return::CstStatReturn,
    location::Location, parser::Parser, temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_return(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;
    self.next_lexeme();

    let mut list = TempVector::new(&mut self.scratch_expr);
    let mut comma_positions = TempVector::new(&mut self.scratch_position);

    if !self.block_follow(self.lexer.current()) && self.lexer.current().r#type != Type::SEMICOLON {
      self.parse_expr_list(
        &mut list,
        if self.options.store_cst_data {
          Some(&mut comma_positions)
        } else {
          None
        },
      );
    }

    let end = if let Some(&expr) = list.last() {
      // Safety: `expr` 是 parse_expr_list 收集的非空 `*mut AstExpr`，指向 arena 存活节点。
      unsafe { (*expr).base.location }
    } else {
      start
    };

    let list_array = self.copy_temp_vector_t(&list);
    let node = self.alloc_stat(AstStatReturn::new(
      Location::new(start.begin, end.end),
      list_array,
    ));

    if self.options.store_cst_data {
      let comma_positions_array = self.copy_temp_vector_t(&comma_positions);
      self.attach_cst(node, |alloc| {
        alloc.alloc(CstStatReturn::new(comma_positions_array))
      });
    }

    if LuauExportValueSyntax.get() && self.function_stack.len() == 1 {
      // cpp:1447-1451：冲突仅 report（返回原 AstStatReturn 节点），
      // hasModuleReturn 无条件置位
      if !self.declared_export_bindings.is_empty() {
        // Safety: `node` 由 alloc_stat 返回、指向 arena 存活的 `AstStatReturn`（非空、地址稳定），
        // 仅读其基类 location。
        self.report(
          // Safety: 同上
          unsafe { (*node).base.location },
          format_args!(
            "Exporting values is not compatible with top-level return (export/return conflict)"
          ),
        );
      }

      self.has_module_return = true;
    }

    node
  }
}
