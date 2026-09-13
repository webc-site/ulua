use ulua_ast::records::ast_stat_assign::AstStatAssign;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::extract_l_value_symbol::extract_l_value_symbol,
  records::{assign::Assign, cfg_builder::CfgBuilder},
};
impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `assn` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn lower_ast_stat_assign(&mut self, assn: *mut AstStatAssign) {
    unsafe {
      for &expr in (*assn).values.as_slice() {
        self.lower_expr_ast_expr(expr);
      }

      for &target in (*assn).vars.as_slice() {
        // C++:
        //   if (auto sym = extractLValueSymbol(target)) {
        //       DefId def = newDefinition(*sym);
        //       emit<Assign>(currentBlock, def, assn);
        //       currentBlock->setReachingDefinition(*sym, def);
        //   } else LUAU_ASSERT(!"Unhandled lvalue type");
        if let Some(sym) = extract_l_value_symbol(&*target) {
          let def = self.new_definition(sym.clone());
          let current_block = self.current_block;
          self.emit::<Assign, _>(current_block, (def, assn));
          (*current_block).set_reaching_definition(sym, def);
        } else {
          LUAU_ASSERT!(false);
        }
      }
    }
  }
}
