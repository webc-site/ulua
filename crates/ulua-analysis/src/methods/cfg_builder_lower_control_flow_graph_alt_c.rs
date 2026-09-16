use core::ptr::null_mut;

use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::{
  records::{cfg_builder::CfgBuilder, declare::Declare, symbol::Symbol},
  type_aliases::def_id_control_flow_graph::DefId,
};
impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `local` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn lower_ast_stat_local(&mut self, local: *mut AstStatLocal) {
    unsafe {
      let local_ref = &*local;
      let values = local_ref.values.as_slice();

      for (i, &loc) in local_ref.vars.as_slice().iter().enumerate() {
        let expr = values.get(i).copied().unwrap_or(null_mut());

        if !expr.is_null() {
          self.lower_expr_ast_expr(expr);
        }

        // C++:
        //   Symbol sym(loc);
        //   DefId def = newDefinition(sym);
        //   emit<Declare>(currentBlock, def, local);
        //   currentBlock->setReachingDefinition(sym, def);
        let sym = Symbol::from_local(loc);
        let def: DefId = self.new_definition(sym.clone());
        let current_block = self.current_block;
        self.emit::<Declare, _>(current_block, (def, local));
        (*current_block).set_reaching_definition(sym, def);
      }
    }
  }
}
