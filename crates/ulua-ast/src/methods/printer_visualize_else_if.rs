use crate::{
  records::{ast_stat_if::AstStatIf, printer::Printer, writer::Writer},
  rtti::ast_node_try_as_mut,
};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_else_if(&mut self, elseif: &mut AstStatIf) {
    self.visualize_ast_expr(unsafe { &mut *elseif.condition });

    if let Some(ref loc) = elseif.then_location {
      self.advance(loc.begin);
    }

    self.writer.keyword("then");

    self.visualize_block_ast_stat_block(unsafe { &mut *elseif.thenbody });

    if elseif.elsebody.is_null() {
      self.advance(unsafe { (*elseif.thenbody).base.base.location.end });
      self.writer.keyword("end");
    } else if let Some(elseifelseif) =
      unsafe { ast_node_try_as_mut::<AstStatIf>(elseif.elsebody as *mut _) }
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

      self.visualize_block_ast_stat(unsafe { &mut *elseif.elsebody });
      self.advance(unsafe { (*elseif.elsebody).base.location.end });
      self.writer.keyword("end");
    }
  }
}
