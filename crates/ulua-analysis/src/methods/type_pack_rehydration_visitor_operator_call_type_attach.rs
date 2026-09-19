use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::{
  records::type_pack_rehydration_visitor::TypePackRehydrationVisitor,
  type_aliases::bound_type_pack::BoundTypePack,
};

impl TypePackRehydrationVisitor {
  /// C++ `AstTypePack* operator()(const BoundTypePack& btp) const` —
  /// `return Luau::visit(*this, btp.bound_to->ty);`.
  pub fn operator_call_2(&self, btp: &BoundTypePack) -> *mut AstTypePack {
    unsafe { self.visit_type_pack(btp.bound_to) }
  }
}
