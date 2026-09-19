use crate::{
  records::{ast_stat_if::AstStatIf, printer::Printer, writer::Writer},
  rtti::ast_node_try_as,
};

impl<'a, W: Writer> Printer<'a, W> {
  /// cpp `Printer::visit(AstStatIf* elseif)` 的 else-if 分支：打印器只写
  /// `Writer`，节点全程共享借用。
  pub fn visualize_else_if(&mut self, elseif: &AstStatIf) {
    self.visualize_ast_expr(unsafe { &*elseif.condition });

    if let Some(ref loc) = elseif.then_location {
      self.advance(loc.begin);
    }

    self.writer.keyword("then");

    self.visualize_block_ast_stat_block(unsafe { &*elseif.thenbody });

    if elseif.elsebody.is_null() {
      self.advance(unsafe { (*elseif.thenbody).base.base.location.end });
      self.writer.keyword("end");
    } else if let Some(elseifelseif) =
      ast_node_try_as::<AstStatIf>(unsafe { &(*elseif.elsebody).base })
    {
      if let Some(ref loc) = elseif.else_location {
        self.advance(loc.begin);
      }
      self.writer.keyword("elseif");
      self.visualize_else_if(elseifelseif);
    } else {
      if let Some(ref loc) = elseif.else_location {
        self.advance(loc.begin);
      }
      self.writer.keyword("else");

      self.visualize_block_ast_stat(unsafe { &*elseif.elsebody });
      self.advance(unsafe { (*elseif.elsebody).base.location.end });
      self.writer.keyword("end");
    }
  }
}
