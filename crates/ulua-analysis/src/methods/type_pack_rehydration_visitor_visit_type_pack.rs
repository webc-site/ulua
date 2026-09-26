//! Source: `Analysis/src/TypeAttach.cpp` (the `Luau::visit(*this, TypePackId->ty)`
//! overload dispatch over `TypePackVariant`).
//!
//! C++ `Luau::visit(tprv, tp->ty)` selects the `operator()` overload matching
//! the active pack alternative. The Rust port is a `match` over the variant
//! calling the pinned `operator_call_N` arm per member.

use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::{
  functions::get_type_pack::type_pack_variant_of,
  records::type_pack_rehydration_visitor::TypePackRehydrationVisitor,
  type_aliases::{
    bound_type_pack::BoundTypePack, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};

impl TypePackRehydrationVisitor {
  pub fn visit_type_pack(&self, tp: TypePackId) -> *mut AstTypePack {
    // 变体读取收口在 `type_pack_variant_of`（arena 节点有效性契约同 C++ get）。
    match type_pack_variant_of(tp) {
      TypePackVariant::Bound(b) => {
        // C++ `operator()(const BoundTypePack& btp)` returns
        // `Luau::visit(*this, btp.bound_to->ty)`.
        let btp = BoundTypePack { bound_to: *b };
        self.rehydrate_bound_pack(&btp)
      }
      TypePackVariant::Error(e) => self.rehydrate_error_pack(e),
      TypePackVariant::Free(f) => self.rehydrate_free_pack(f),
      TypePackVariant::Generic(g) => self.rehydrate_generic_pack(g),
      TypePackVariant::TypePack(t) => self.rehydrate_type_pack(t),
      TypePackVariant::Variadic(v) => self.rehydrate_variadic_pack(v),
      TypePackVariant::Blocked(b) => self.rehydrate_blocked_pack(b),
      TypePackVariant::TypeFunctionInstance(t) => self.rehydrate_type_function_instance_pack(t),
    }
  }
}
