//! `type_rehydration_visitor` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use std::collections::BTreeMap;

use ulua_ast::records::{allocator::Allocator, ast_type::AstType, ast_type_pack::AstTypePack};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type::type_variant_of,
  records::{
    bound::Bound, type_pack_rehydration_visitor::TypePackRehydrationVisitor,
    type_rehydration_options::TypeRehydrationOptions,
    type_rehydration_visitor::TypeRehydrationVisitor, visit_key::VisitKey,
  },
  type_aliases::{
    synthetic_names::SyntheticNames, type_id::TypeId, type_pack_id::TypePackId,
    type_variant::TypeVariant,
  },
};

impl TypeRehydrationVisitor {
  /// C++ `bool hasSeen(const void* tv)`.
  pub fn has_seen(&mut self, tv: *const ()) -> bool {
    let key = VisitKey::from_ptr(tv);
    if let Some(&count) = self.seen.get(&key)
      && count < self.count
    {
      return true;
    }

    self.seen.insert(key, self.count);
    false
  }
}

impl TypeRehydrationVisitor {
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
    tprv.visit_type_pack(tp)
  }
}

impl TypeRehydrationVisitor {
  #[inline]
  pub fn type_rehydration_visitor_type_rehydration_visitor(
    alloc: *mut Allocator,
    synthetic_names: *mut SyntheticNames,
    options: &TypeRehydrationOptions,
  ) -> Self {
    Self {
      seen: BTreeMap::new(),
      count: 0,
      allocator: alloc,
      synthetic_names,
      options: options.clone(),
    }
  }
}

// Source: `Analysis/src/TypeAttach.cpp` (the `Luau::visit(*this, TypeId->ty)`
// overload dispatch over `TypeVariant`, faithful to `AstType* Luau::visit`).
//
// C++ `Luau::visit(visitor, type->ty)` is the std::variant visitor dispatch:
// it selects the `operator()` overload matching the active alternative. The
// Rust port is a `match` over the variant that calls the pinned
// `operator_call_N` arm per member (same idiom as
// `type_stringifier_stringify_to_string.rs`).

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
