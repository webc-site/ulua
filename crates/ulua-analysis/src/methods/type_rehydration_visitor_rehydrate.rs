use ulua_ast::records::ast_type_pack::AstTypePack;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    type_pack_rehydration_visitor::TypePackRehydrationVisitor,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl TypeRehydrationVisitor {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn rehydrate(&mut self, tp: TypePackId) -> *mut AstTypePack {
    LUAU_ASSERT!(!tp.is_null());

    let type_visitor = self as *mut TypeRehydrationVisitor;
    let tprv =
      TypePackRehydrationVisitor::type_pack_rehydration_visitor_type_pack_rehydration_visitor(
        self.allocator,
        self.synthetic_names,
        type_visitor,
      );

    // C++ `Luau::visit(tprv, tp->ty)` — dispatch over the pack variant.
    unsafe { tprv.visit_type_pack(tp) }
  }
}
