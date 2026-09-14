//! Source: `Analysis/include/Luau/VisitType.h` (GenericTypeVisitor::traverse, L217-505)
//!
//! Faithful traversal driver for `SkipCacheForType`. The C++ visitor relies on
//! `GenericTypeVisitor::traverse(TypeId/TypePackId)` to (a) dispatch to the
//! per-variant `visit(...)` overrides and (b) recurse into composite types so a
//! mutable element nested anywhere in the type tree is found. The standalone
//! Rust `SkipCacheForType` previously only invoked the generic `visit(TypeId)`
//! fallback, so unsealed/free tables (and any nested mutable type) were never
//! flagged and `canCacheResult` wrongly cached unifications involving them.
//!
//! `SkipCacheForType` is a `TypeOnceVisitor` constructed with
//! `skipBoundTypes = false`, so bound types are visited then followed.

use std::collections::HashSet;

use crate::{
  records::skip_cache_for_type::SkipCacheForType,
  type_aliases::{
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
    type_variant::TypeVariant,
  },
};

impl SkipCacheForType {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn traverse_skip_cache(
    &mut self,
    ty: TypeId,
    seen_types: &mut HashSet<*const ()>,
    seen_packs: &mut HashSet<*const ()>,
  ) {
    // Once we have concluded that the type is uncacheable there is no point
    // continuing the walk (mirrors the early-out the C++ visitor achieves by
    // returning false from each `visit` that sets `result`).
    if self.result {
      return;
    }

    let key = ty as *const ();
    if seen_types.contains(&key) {
      return;
    }
    seen_types.insert(key);

    match unsafe { &(*ty).ty } {
      TypeVariant::Bound(bound_to) => {
        // skipBoundTypes is false for this visitor: there is no
        // dedicated `visit(ty, BoundType&)` override that changes the
        // recursion, so traverse through to the bound target. (The base
        // `visit(ty, BoundType&)` returns true.)
        let target = *bound_to;
        unsafe { self.traverse_skip_cache(target, seen_types, seen_packs) };
      }
      TypeVariant::Free(ft) => {
        self.visit_type_id_free_type(ty, ft);
      }
      TypeVariant::Generic(gt) => {
        self.visit_type_id_generic_type(ty, gt);
      }
      TypeVariant::Error(_) => {
        // base visit(ty, ErrorType&) — no children
      }
      TypeVariant::Primitive(_)
      | TypeVariant::Singleton(_)
      | TypeVariant::Any(_)
      | TypeVariant::Unknown(_)
      | TypeVariant::Never(_)
      | TypeVariant::NoRefine(_) => {
        // Leaf types; base visit returns true but there are no children.
      }
      TypeVariant::Function(ftv) => {
        if self.visit_type_id(ty) {
          let arg_types = ftv.arg_types();
          let ret_types = ftv.ret_types();
          self.traverse_skip_cache_pack(arg_types, seen_types, seen_packs);
          self.traverse_skip_cache_pack(ret_types, seen_types, seen_packs);
        }
      }
      TypeVariant::Table(ttv) => {
        if unsafe { self.visit_type_id_table_type(ty, ttv) } {
          if let Some(bound_to) = ttv.bound_to {
            unsafe { self.traverse_skip_cache(bound_to, seen_types, seen_packs) };
          } else {
            for prop in ttv.props.values() {
              if let Some(read) = prop.read_ty {
                unsafe { self.traverse_skip_cache(read, seen_types, seen_packs) };
              }
              if let Some(write) = prop.write_ty
                && !prop.is_shared()
              {
                unsafe { self.traverse_skip_cache(write, seen_types, seen_packs) };
              }
            }
            if let Some(indexer) = &ttv.indexer {
              unsafe { self.traverse_skip_cache(indexer.index_type, seen_types, seen_packs) };
              unsafe {
                self.traverse_skip_cache(indexer.index_result_type, seen_types, seen_packs)
              };
            }
          }
        }
      }
      TypeVariant::Metatable(mtv) => {
        if self.visit_type_id(ty) {
          unsafe { self.traverse_skip_cache(mtv.table(), seen_types, seen_packs) };
          unsafe { self.traverse_skip_cache(mtv.metatable(), seen_types, seen_packs) };
        }
      }
      TypeVariant::Extern(etv) => {
        if self.visit_type_id(ty) {
          for prop in etv.props.values() {
            if let Some(read) = prop.read_ty {
              unsafe { self.traverse_skip_cache(read, seen_types, seen_packs) };
            }
            if let Some(write) = prop.write_ty
              && !prop.is_shared()
            {
              unsafe { self.traverse_skip_cache(write, seen_types, seen_packs) };
            }
          }
          if let Some(parent) = etv.parent {
            unsafe { self.traverse_skip_cache(parent, seen_types, seen_packs) };
          }
          if let Some(metatable) = etv.metatable {
            unsafe { self.traverse_skip_cache(metatable, seen_types, seen_packs) };
          }
          if let Some(indexer) = &etv.indexer {
            unsafe { self.traverse_skip_cache(indexer.index_type, seen_types, seen_packs) };
            unsafe { self.traverse_skip_cache(indexer.index_result_type, seen_types, seen_packs) };
          }
        }
      }
      TypeVariant::Union(utv) => {
        if self.visit_type_id(ty) {
          for opt_ty in utv.options.iter() {
            unsafe { self.traverse_skip_cache(*opt_ty, seen_types, seen_packs) };
          }
        }
      }
      TypeVariant::Intersection(itv) => {
        if self.visit_type_id(ty) {
          for part_ty in itv.parts.iter() {
            unsafe { self.traverse_skip_cache(*part_ty, seen_types, seen_packs) };
          }
        }
      }
      TypeVariant::Lazy(ltv) => {
        let unwrapped = ltv.unwrapped;
        if !unwrapped.is_null() {
          unsafe { self.traverse_skip_cache(unwrapped as TypeId, seen_types, seen_packs) };
        }
        // Visiting into an un-unwrapped LazyType could cause infinite
        // expansion, so we don't (matches C++).
      }
      TypeVariant::Blocked(bt) => {
        self.visit_type_id_blocked_type(ty, bt);
      }
      TypeVariant::PendingExpansion(petv) => {
        if self.visit_type_id_pending_expansion_type(ty, petv) {
          for a in petv.type_arguments.iter() {
            unsafe { self.traverse_skip_cache(*a, seen_types, seen_packs) };
          }
          for a in petv.pack_arguments.iter() {
            self.traverse_skip_cache_pack(*a, seen_types, seen_packs);
          }
        }
      }
      TypeVariant::Negation(ntv) => {
        if self.visit_type_id(ty) {
          unsafe { self.traverse_skip_cache(ntv.ty, seen_types, seen_packs) };
        }
      }
      TypeVariant::TypeFunctionInstance(tfit) => {
        if self.visit_type_id(ty) {
          for p in tfit.type_arguments.iter() {
            unsafe { self.traverse_skip_cache(*p, seen_types, seen_packs) };
          }
          for p in tfit.pack_arguments.iter() {
            self.traverse_skip_cache_pack(*p, seen_types, seen_packs);
          }
        }
      }
    }

    seen_types.remove(&key);
  }

  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn traverse_skip_cache_pack(
    &mut self,
    tp: TypePackId,
    seen_types: &mut HashSet<*const ()>,
    seen_packs: &mut HashSet<*const ()>,
  ) {
    if self.result {
      return;
    }

    let key = tp as *const ();
    if seen_packs.contains(&key) {
      return;
    }
    seen_packs.insert(key);

    match unsafe { &(*tp).ty } {
      TypePackVariant::Bound(bound_to) => {
        // base visit(tp, BoundTypePack&) returns true → recurse
        let target = *bound_to;
        self.traverse_skip_cache_pack(target, seen_types, seen_packs);
      }
      TypePackVariant::Free(ftp) => {
        self.visit_type_pack_id_free_type_pack(tp, ftp);
      }
      TypePackVariant::Generic(gtp) => {
        self.visit_type_pack_id_generic_type_pack(tp, gtp);
      }
      TypePackVariant::Error(_) => {
        // base visit(tp, ErrorTypePack&) — no children
      }
      TypePackVariant::TypePack(pack) => {
        if unsafe { self.visit_type_pack_id(tp) } {
          for ty in pack.head.iter() {
            unsafe { self.traverse_skip_cache(*ty, seen_types, seen_packs) };
          }
          if let Some(tail) = pack.tail {
            self.traverse_skip_cache_pack(tail, seen_types, seen_packs);
          }
        }
      }
      TypePackVariant::Variadic(pack) => {
        if unsafe { self.visit_type_pack_id(tp) } {
          unsafe { self.traverse_skip_cache(pack.ty, seen_types, seen_packs) };
        }
      }
      TypePackVariant::Blocked(btp) => {
        self.visit_type_pack_id_blocked_type_pack(tp, btp);
      }
      TypePackVariant::TypeFunctionInstance(tfitp) => {
        if unsafe { self.visit_type_pack_id(tp) } {
          for t in tfitp.type_arguments.iter() {
            unsafe { self.traverse_skip_cache(*t, seen_types, seen_packs) };
          }
          for t in tfitp.pack_arguments.iter() {
            self.traverse_skip_cache_pack(*t, seen_types, seen_packs);
          }
        }
      }
    }

    seen_packs.remove(&key);
  }
}
