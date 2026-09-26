//! Source: `Analysis/src/TypeAttach.cpp` (the `Luau::visit(*this, TypeId->ty)`
//! overload dispatch over `TypeVariant`, faithful to `AstType* Luau::visit`).
//!
//! C++ `Luau::visit(visitor, type->ty)` is the std::variant visitor dispatch:
//! it selects the `operator()` overload matching the active alternative. The
//! Rust port is a `match` over the variant that calls the pinned
//! `operator_call_N` arm per member (same idiom as
//! `type_stringifier_stringify_to_string.rs`).

use ulua_ast::records::ast_type::AstType;

use crate::{
  functions::get_type::type_variant_of,
  records::{bound::Bound, type_rehydration_visitor::TypeRehydrationVisitor},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
impl TypeRehydrationVisitor {
  pub fn visit_type(&mut self, ty: TypeId) -> *mut AstType {
    // 变体读取收口在 `type_variant_of`（arena 节点有效性契约同 C++ get）。
    match type_variant_of(ty) {
      TypeVariant::Bound(b) => {
        // C++ `operator()(const Unifiable::Bound<TypeId>& bound)` takes
        // the Bound wrapper; the variant stores the bare TypeId, so
        // rebuild the Bound around it.
        let bound = Bound { bound_to: *b };
        self.rehydrate_bound(&bound)
      }
      TypeVariant::Error(e) => self.rehydrate_error(e),
      TypeVariant::Free(f) => self.rehydrate_free(f),
      TypeVariant::Generic(g) => self.rehydrate_generic(g),
      TypeVariant::Primitive(p) => self.rehydrate_primitive(p),
      TypeVariant::Singleton(s) => self.rehydrate_singleton(s),
      TypeVariant::Blocked(b) => self.rehydrate_blocked(b),
      TypeVariant::PendingExpansion(p) => self.rehydrate_pending_expansion(p),
      TypeVariant::Function(f) => self.rehydrate_function(f),
      TypeVariant::Table(t) => self.rehydrate_table(t),
      TypeVariant::Metatable(m) => self.rehydrate_metatable(m),
      TypeVariant::Extern(e) => self.rehydrate_extern(e),
      TypeVariant::Any(a) => self.rehydrate_any(a),
      TypeVariant::Union(u) => self.rehydrate_union(u),
      TypeVariant::Intersection(i) => self.rehydrate_intersection(i),
      TypeVariant::Lazy(l) => self.rehydrate_lazy(l),
      TypeVariant::Unknown(u) => self.rehydrate_unknown(u),
      TypeVariant::Never(n) => self.rehydrate_never(n),
      TypeVariant::Negation(n) => self.rehydrate_negation(n),
      TypeVariant::NoRefine(n) => self.rehydrate_no_refine(n),
      TypeVariant::TypeFunctionInstance(t) => self.rehydrate_type_function_instance(t),
    }
  }
}
