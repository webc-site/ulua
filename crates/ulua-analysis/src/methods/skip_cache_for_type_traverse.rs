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
//! `skipBoundTypes = false`; per C++ Unifier.cpp，命中 Bound（类型或 pack）
//! 即判不可缓存并停止递归，不穿透到 bound 目标。

use crate::{
  functions::{get_type::type_variant_of, get_type_pack::type_pack_variant_of},
  records::skip_cache_for_type::SkipCacheForType,
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, collections::HashSet, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};

impl SkipCacheForType {
  /// 递归包装：把 `traverse_skip_cache` 的调用点收敛到一处。
  #[inline]
  fn recurse_ty(
    &mut self,
    ty: TypeId,
    seen_types: &mut HashSet<*const ()>,
    seen_packs: &mut HashSet<*const ()>,
  ) {
    self.traverse_skip_cache(ty, seen_types, seen_packs)
  }

  /// arena 节点有效性契约见 [`crate::functions::get_type::type_variant_of`]
  /// （`ty` 须指向遍历期间稳定存活的 arena `Type`，遍历全程不改写类型图）。
  pub fn traverse_skip_cache(
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

    // 变体读取收口在 `type_variant_of`（arena 节点有效性契约同 C++ get）：
    // 产生的共享引用贯穿整个 `match`，臂内对 `&mut self` 的递归只改 visitor 的
    // `result`/seen 集合，与类型 arena 存储不相交。
    match type_variant_of(ty) {
      TypeVariant::Bound(bound_to) => {
        // C++ `SkipCacheForType::visit(TypeId, const BoundType&)`（Unifier.cpp:167）：
        // 命中 Bound 即判不可缓存（result = true）并停止，不穿透到 bound 目标。
        let btv = BoundType {
          bound_to: *bound_to,
        };
        self.visit_type_id_bound_type(ty, &btv);
      }
      TypeVariant::Free(ft) => {
        self.visit_type_id_free_type(ty, ft);
      }
      TypeVariant::Generic(gt) => {
        self.visit_type_id_generic_type(ty, gt);
      }
      TypeVariant::Error(_) => {
        // C++ 默认 `visit(ty, ErrorType&)` → 基类 `visit(ty)`：查 skipCacheForType
        // 表，命中则标记不可缓存；leaf 无子节点，返回值不参与控制流。
        self.visit_type_id(ty);
      }
      TypeVariant::Primitive(_)
      | TypeVariant::Singleton(_)
      | TypeVariant::Any(_)
      | TypeVariant::Unknown(_)
      | TypeVariant::Never(_)
      | TypeVariant::NoRefine(_) => {
        // C++ 同样走基类 `visit(ty)` 的 skipCacheForType 表查询；leaf 无子节点。
        self.visit_type_id(ty);
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
        // visit_type_id_table_type 已降 safe：被调方对 `(*ty).owning_arena` 的
        // 再读由其内部窄块收口，与 `ttv` 共享借用并存的仅只读访问。
        if self.visit_type_id_table_type(ty, ttv) {
          if let Some(bound_to) = ttv.bound_to {
            self.recurse_ty(bound_to, seen_types, seen_packs);
          } else {
            for prop in ttv.props.values() {
              if let Some(read) = prop.read_ty {
                self.recurse_ty(read, seen_types, seen_packs);
              }
              if let Some(write) = prop.write_ty
                && !prop.is_shared()
              {
                self.recurse_ty(write, seen_types, seen_packs);
              }
            }
            if let Some(indexer) = &ttv.indexer {
              self.recurse_ty(indexer.index_type, seen_types, seen_packs);
              self.recurse_ty(indexer.index_result_type, seen_types, seen_packs);
            }
          }
        }
      }
      TypeVariant::Metatable(mtv) => {
        if self.visit_type_id(ty) {
          self.recurse_ty(mtv.table(), seen_types, seen_packs);
          self.recurse_ty(mtv.metatable(), seen_types, seen_packs);
        }
      }
      TypeVariant::Extern(etv) => {
        if self.visit_type_id(ty) {
          for prop in etv.props.values() {
            if let Some(read) = prop.read_ty {
              self.recurse_ty(read, seen_types, seen_packs);
            }
            if let Some(write) = prop.write_ty
              && !prop.is_shared()
            {
              self.recurse_ty(write, seen_types, seen_packs);
            }
          }
          if let Some(parent) = etv.parent {
            self.recurse_ty(parent, seen_types, seen_packs);
          }
          if let Some(metatable) = etv.metatable {
            self.recurse_ty(metatable, seen_types, seen_packs);
          }
          if let Some(indexer) = &etv.indexer {
            self.recurse_ty(indexer.index_type, seen_types, seen_packs);
            self.recurse_ty(indexer.index_result_type, seen_types, seen_packs);
          }
        }
      }
      TypeVariant::Union(utv) => {
        if self.visit_type_id(ty) {
          for opt_ty in utv.options.iter() {
            self.recurse_ty(*opt_ty, seen_types, seen_packs);
          }
        }
      }
      TypeVariant::Intersection(itv) => {
        if self.visit_type_id(ty) {
          for part_ty in itv.parts.iter() {
            self.recurse_ty(*part_ty, seen_types, seen_packs);
          }
        }
      }
      TypeVariant::Lazy(ltv) => {
        let unwrapped = ltv.unwrapped;
        if !unwrapped.is_null() {
          // Safety: `LazyType::unwrapped` 由 lazy 展开写入的是 arena 中存活的
          // `Type` 指针（cpp `LazyType::unwrapped` 不变量），null 已在上文短路，
          // 满足 `recurse_ty` 的存活 TypeId 契约。
          self.recurse_ty(unwrapped as TypeId, seen_types, seen_packs);
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
            self.recurse_ty(*a, seen_types, seen_packs);
          }
          for a in petv.pack_arguments.iter() {
            self.traverse_skip_cache_pack(*a, seen_types, seen_packs);
          }
        }
      }
      TypeVariant::Negation(ntv) => {
        if self.visit_type_id(ty) {
          self.recurse_ty(ntv.ty, seen_types, seen_packs);
        }
      }
      TypeVariant::TypeFunctionInstance(tfit) => {
        if self.visit_type_id(ty) {
          for p in tfit.type_arguments.iter() {
            self.recurse_ty(*p, seen_types, seen_packs);
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
  /// - `tp` 必须非空并指向 `TypeArena::type_packs` 分配的存活 `TypePackVar`
  ///   （`TypePackId = *const TypePackVar`，cpp `traverse(TypePackId)` 对
  ///   pack 的同一「须在 arena 中」前提）：本文件内实参都取自存活类型节点的
  ///   子节点或 `unifier_can_cache_result` 的 unify 输入，遍历期间 arena 不
  ///   释放、不改写其 `ty` 字段。
  /// - `seen_types`/`seen_packs` 须为本次 traversal 独占的临时集合，与
  ///   [`Self::traverse_skip_cache`] 的契约同源。
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

    // 变体读取收口在 `type_pack_variant_of`（arena 节点契约同 C++ get）：共享
    // 引用贯穿整个 `match`，臂内递归只改 `self.result` 与 seen 集合。
    match type_pack_variant_of(tp) {
      TypePackVariant::Bound(bound_to) => {
        // C++ `SkipCacheForType::visit(TypePackId, const BoundTypePack&)`（Unifier.cpp:247）：
        // 命中 Bound pack 即判不可缓存（result = true）并停止，不穿透。
        let btp = BoundTypePack {
          bound_to: *bound_to,
        };
        self.visit_type_pack_id_bound_type_pack(tp, &btp);
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
        // visit_type_pack_id 已降 safe：被调方只读 `owning_arena` 身份字段与本
        // visitor 的 arena 归属比对，句柄前提在其窄块内证成。
        if self.visit_type_pack_id(tp) {
          for ty in pack.head.iter() {
            self.recurse_ty(*ty, seen_types, seen_packs);
          }
          if let Some(tail) = pack.tail {
            self.traverse_skip_cache_pack(tail, seen_types, seen_packs);
          }
        }
      }
      TypePackVariant::Variadic(pack) => {
        // 同 TypePack 臂：被调已降 safe，仅 owning_arena 归属比对。
        if self.visit_type_pack_id(tp) {
          self.recurse_ty(pack.ty, seen_types, seen_packs);
        }
      }
      TypePackVariant::Blocked(btp) => {
        self.visit_type_pack_id_blocked_type_pack(tp, btp);
      }
      TypePackVariant::TypeFunctionInstance(tfitp) => {
        // 同 TypePack 臂：被调已降 safe，仅 owning_arena 归属比对。
        if self.visit_type_pack_id(tp) {
          for t in tfitp.type_arguments.iter() {
            self.recurse_ty(*t, seen_types, seen_packs);
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
