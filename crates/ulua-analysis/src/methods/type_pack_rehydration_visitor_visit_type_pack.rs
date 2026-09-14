//! Node: `cxx:Method:Luau.Analysis:Analysis/src/TypeAttach.cpp:visit`
//! Source: `Analysis/src/TypeAttach.cpp` (the `Luau::visit(*this, TypePackId->ty)`
//! overload dispatch over `TypePackVariant`).
//!
//! C++ `Luau::visit(tprv, tp->ty)` selects the `operator()` overload matching
//! the active pack alternative. The Rust port is a `match` over the variant
//! calling the pinned `operator_call_N` arm per member.

use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::{
  records::type_pack_rehydration_visitor::TypePackRehydrationVisitor,
  type_aliases::{
    bound_type_pack::BoundTypePack, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};

impl TypePackRehydrationVisitor {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn visit_type_pack(&self, tp: TypePackId) -> *mut AstTypePack {
    match unsafe { &(*tp).ty } {
      TypePackVariant::Bound(b) => {
        // C++ `operator()(const BoundTypePack& btp)` returns
        // `Luau::visit(*this, btp.bound_to->ty)`.
        let btp = BoundTypePack { bound_to: *b };
        self.operator_call_2(&btp)
      }
      TypePackVariant::Error(e) => self.operator_call_3(e),
      TypePackVariant::Free(f) => self.operator_call_4(f),
      TypePackVariant::Generic(g) => self.operator_call_5(g),
      TypePackVariant::TypePack(t) => self.operator_call_7(t),
      TypePackVariant::Variadic(v) => self.operator_call_8(v),
      TypePackVariant::Blocked(b) => self.operator_call(b),
      TypePackVariant::TypeFunctionInstance(t) => self.operator_call_6(t),
    }
  }
}
