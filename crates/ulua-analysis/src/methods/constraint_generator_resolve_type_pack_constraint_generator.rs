use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::{
  enums::polarity::Polarity,
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::type_pack_id::TypePackId,
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `scope、`tp` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
    &mut self,
    scope: *mut Scope,
    tp: *mut AstTypePack,
    in_type_argument: bool,
    replace_error_with_fresh: bool,
    initial_polarity: Polarity,
  ) -> TypePackId {
    let _polarity = initial_polarity;
    unsafe {
      self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
        scope,
        tp,
        in_type_argument,
        replace_error_with_fresh,
      )
    }
  }
}
