//! Node: `cxx:Method:Luau.Analysis:Analysis/src/TypeAttach.cpp:visit`
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
  records::{bound::Bound, type_rehydration_visitor::TypeRehydrationVisitor},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
impl TypeRehydrationVisitor {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn visit_type(&mut self, ty: TypeId) -> *mut AstType {
    match unsafe { &(*ty).ty } {
      TypeVariant::Bound(b) => {
        // C++ `operator()(const Unifiable::Bound<TypeId>& bound)` takes
        // the Bound wrapper; the variant stores the bare TypeId, so
        // rebuild the Bound around it.
        let bound = Bound { bound_to: *b };
        self.operator_call_19(&bound)
      }
      TypeVariant::Error(e) => self.operator_call_3(e),
      TypeVariant::Free(f) => self.operator_call_5(f),
      TypeVariant::Generic(g) => self.operator_call_7(g),
      TypeVariant::Primitive(p) => self.operator_call_15(p),
      TypeVariant::Singleton(s) => self.operator_call_16(s),
      TypeVariant::Blocked(b) => self.operator_call_2(b),
      TypeVariant::PendingExpansion(p) => self.operator_call_14(p),
      TypeVariant::Function(f) => self.operator_call_6(f),
      TypeVariant::Table(t) => self.operator_call_17(t),
      TypeVariant::Metatable(m) => self.operator_call_10(m),
      TypeVariant::Extern(e) => self.operator_call_4(e),
      TypeVariant::Any(a) => self.operator_call(a),
      TypeVariant::Union(u) => self.operator_call_20(u),
      TypeVariant::Intersection(i) => self.operator_call_8(i),
      TypeVariant::Lazy(l) => self.operator_call_9(l),
      TypeVariant::Unknown(u) => self.operator_call_21(u),
      TypeVariant::Never(n) => self.operator_call_12(n),
      TypeVariant::Negation(n) => self.operator_call_11(n),
      TypeVariant::NoRefine(n) => self.operator_call_13(n),
      TypeVariant::TypeFunctionInstance(t) => self.operator_call_18(t),
    }
  }
}
