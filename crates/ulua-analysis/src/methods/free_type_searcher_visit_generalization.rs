use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    follow_type, follow_type_pack, get_type::type_variant_of, get_type_pack::type_pack_variant_of,
    has_seen_visit_type::has_seen, subsumes_scope::subsumes, unsee_visit_type::unsee,
  },
  records::{
    arena_handle::alias_opt, extern_type::ExternType, free_type::FreeType,
    free_type_pack::FreeTypePack, free_type_searcher::FreeTypeSearcher,
    function_type::FunctionType, table_type::TableType,
  },
  type_aliases::{
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
    type_variant::TypeVariant,
  },
};

impl FreeTypeSearcher<'_> {
  /// C++ `bool FreeTypeSearcher::visit(TypeId ty)` (Generalization.cpp:91-98) —
  /// the generic dispatch override. Guards re-traversal via the cached-type
  /// set and the polarity-aware seen set; returns `true` so the base
  /// `traverse` descends into children for the variants the searcher does not
  /// specialize.
  pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
    if self.cached_types.contains(&ty) || self.seen_with_current_polarity(ty) {
      return false;
    }

    LUAU_ASSERT!(!ty.is_null());
    true
  }

  /// C++ `bool FreeTypeSearcher::visit(TypeId ty, const FreeType& ft)`
  /// (Generalization.cpp:100-117).
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, ft: &FreeType) -> bool {
    if !subsumes(Some(self.scope), alias_opt(ft.scope)) {
      return true;
    }

    // GeneralizationParams<TypeId>& params = types[ty]; ++params.useCount;
    self.types.get_or_default(ty).use_count += 1;

    if self.cached_types.contains(&ty) || self.seen_with_current_polarity(ty) {
      return false;
    }

    let is_within_function = self.is_within_function;
    let polarity = self.polarity;
    let params = self.types.get_or_default(ty);

    if !is_within_function {
      params.found_outside_functions = true;
    }

    params.polarity |= polarity;

    true
  }

  /// C++ `bool FreeTypeSearcher::visit(TypeId ty, const TableType& tt)`
  /// (Generalization.cpp:119-177).
  pub fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
    if self.cached_types.contains(&ty) || self.seen_with_current_polarity(ty) {
      return false;
    }

    if matches!(tt.state, TableState::Free | TableState::Unsealed)
      && subsumes(Some(self.scope), alias_opt(tt.scope))
    {
      self.unsealed_tables.insert(ty);
    }

    for prop in tt.props.values() {
      if prop.is_read_only() {
        searcher_traverse_type_id(
          self,
          prop
            .read_ty
            .expect("is_read_only() 定义蕴含 read_ty 为 Some"),
        );
      } else if prop.is_write_only() {
        traverse_at_polarity(
          self,
          prop
            .write_ty
            .expect("is_write_only() 定义蕴含 write_ty 为 Some"),
          Polarity::Negative,
        );
      } else if prop.is_shared() {
        traverse_at_polarity(
          self,
          prop.read_ty.expect("is_shared() 定义蕴含 read_ty 为 Some"),
          Polarity::Mixed,
        );
      } else {
        searcher_traverse_type_id(
          self,
          prop
            .read_ty
            .expect("else 支为读写不一致态，Property 构造恒保证两侧皆 Some"),
        );
        traverse_at_polarity(
          self,
          prop
            .write_ty
            .expect("else 支为读写不一致态，Property 构造恒保证两侧皆 Some"),
          Polarity::Negative,
        );
      }
    }

    if let Some(indexer) = &tt.indexer {
      // {[K]: V} is equivalent to get/set/iterate; K and V are mixed.
      traverse_at_polarity(self, indexer.index_type, Polarity::Mixed);
      traverse_at_polarity(self, indexer.index_result_type, Polarity::Mixed);
    }

    false
  }
}

/// 临时切换到给定极性 traverse 子类型后恢复原极性。
///
/// 消除各 visit 分支中重复的「save → 覆写 → traverse → restore」片段。
fn traverse_at_polarity(this: &mut FreeTypeSearcher<'_>, ty: TypeId, polarity: Polarity) {
  let saved = this.polarity;
  this.polarity = polarity;
  searcher_traverse_type_id(this, ty);
  this.polarity = saved;
}

/// C++ `GenericTypeVisitor::traverse(TypeId)` (VisitType.h:217-442) specialized
/// for the `FreeTypeSearcher`. Because the visitor's overrides live as inherent
/// methods here (not a trait impl), this reproduces the base traverse contract
/// faithfully: follow bound types (the searcher sets `skipBoundTypes`), apply
/// the recursion-stack `seen` guard *before* dispatching (so a type currently
/// on the stack is not re-visited or re-counted), dispatch to the matching
/// `visit(ty, variant)` override and descend if it returns `true`, then `unsee`
/// the type so sibling paths may visit it again.
///
/// The `seen` guard is distinct from the searcher's polarity-aware
/// `seenWithCurrentPolarity` bookkeeping: `seen` is the base visitor's
/// recursion-stack set (cleared on `unsee`), which is what keeps cyclic types
/// such as `t1 = Instance & { IsA: (t1, ...) -> ... }` from being traversed
/// repeatedly at flipped polarity and inflating `useCount` / polarity.
pub(crate) fn searcher_traverse_type_id(this: &mut FreeTypeSearcher<'_>, ty: TypeId) {
  let ty = follow_type::follow(ty);

  // C++ `if (hasSeen(seen, ty)) { cycle(ty); return; }`. `FreeTypeSearcher`
  // does not override `cycle`, so re-entry is a no-op return.
  if has_seen(&mut this.base.base.seen, ty) {
    return;
  }

  searcher_dispatch_type_id(this, ty);

  unsee(&mut this.base.base.seen, ty);
}

/// Variant dispatch for `searcher_traverse_type_id` (the body of the C++
/// `traverse` switch). `ty` has already been followed and admitted past the
/// recursion-stack `seen` guard.
///
/// Rust 形态：对 arena 变体做单次 `match` 判别（取代 C++ 逐变体 `get<T>` 链式
/// 探测），`type_variant_of` 一次性解引用返回 `'static` 视图，绑定即具名引用。
fn searcher_dispatch_type_id(this: &mut FreeTypeSearcher<'_>, ty: TypeId) {
  let tv = type_variant_of(ty);

  match tv {
    TypeVariant::Free(ft) => {
      if this.visit_type_id_free_type(ty, ft) {
        searcher_traverse_type_id(this, ft.lower_bound);
        searcher_traverse_type_id(this, ft.upper_bound);
      }
      return;
    }
    TypeVariant::Table(tt) => {
      // visit(TypeId, const TableType&) performs its own polarity-aware
      // traversal and returns false.
      this.visit_type_id_table_type(ty, tt);
      return;
    }
    TypeVariant::Function(ft) => {
      // visit(TypeId, const FunctionType&) (Generalization.cpp:179-196)
      // performs its own flipped traversal and returns false.
      if this.visit_type_id_function_type(ty, ft) {
        searcher_traverse_type_pack_id(this, ft.arg_types);
        searcher_traverse_type_pack_id(this, ft.ret_types);
      }
      return;
    }
    TypeVariant::Extern(et) => {
      // visit(TypeId, const ExternType&) -> false (Generalization.cpp:198-201).
      this.visit_type_id_extern_type(ty, et);
      return;
    }
    _ => {}
  }

  // Variants the searcher does not specialize: run the base
  // `visit(TypeId)` guard once, then descend through children.
  if !this.visit_type_id(ty) {
    return;
  }

  match tv {
    TypeVariant::Union(ut) => {
      for &opt in &ut.options {
        searcher_traverse_type_id(this, opt);
      }
    }
    TypeVariant::Intersection(it) => {
      for &part in &it.parts {
        searcher_traverse_type_id(this, part);
      }
    }
    TypeVariant::Metatable(mt) => {
      searcher_traverse_type_id(this, mt.table);
      searcher_traverse_type_id(this, mt.metatable);
    }
    TypeVariant::Negation(nt) => {
      searcher_traverse_type_id(this, nt.ty);
    }
    _ => {}
  }
}

/// C++ `GenericTypeVisitor::traverse(TypePackId)` (VisitType.h:444-505) for the
/// `FreeTypeSearcher`. Same recursion-stack `seen` discipline as the type
/// traversal above.
pub(crate) fn searcher_traverse_type_pack_id(this: &mut FreeTypeSearcher<'_>, tp: TypePackId) {
  let tp = follow_type_pack::follow(tp);

  if has_seen(&mut this.base.base.seen, tp) {
    return;
  }

  searcher_dispatch_type_pack_id(this, tp);

  unsee(&mut this.base.base.seen, tp);
}

/// Variant dispatch for `searcher_traverse_type_pack_id`.
///
/// 与类型侧同构：单次 `match` arena 变体，绑定即具名引用，取代链式 `get<T>` 探测。
fn searcher_dispatch_type_pack_id(this: &mut FreeTypeSearcher<'_>, tp: TypePackId) {
  match type_pack_variant_of(tp) {
    TypePackVariant::Free(ftp) => {
      this.visit_type_pack_id_free_type_pack(tp, ftp);
    }
    TypePackVariant::TypePack(pack) => {
      for &head in &pack.head {
        searcher_traverse_type_id(this, head);
      }
      if let Some(tail) = pack.tail {
        searcher_traverse_type_pack_id(this, tail);
      }
    }
    TypePackVariant::Variadic(vtp) => {
      searcher_traverse_type_id(this, vtp.ty);
    }
    _ => {}
  }
}

impl FreeTypeSearcher<'_> {
  /// C++ `bool FreeTypeSearcher::visit(TypeId ty, const FunctionType& ft)`
  /// (Generalization.cpp:179-196). Flips polarity across the parameter pack:
  /// arguments are traversed in inverted polarity (contravariant), return
  /// types in the current polarity (covariant). Marks the body as being
  /// within a function for the duration. Returns `false` because it performs
  /// its own traversal of the argument and return packs.
  pub fn visit_type_id_function_type(&mut self, ty: TypeId, ft: &FunctionType) -> bool {
    if self.cached_types.contains(&ty) || self.seen_with_current_polarity(ty) {
      return false;
    }

    let old_value = self.is_within_function;
    self.is_within_function = true;

    self.flip();
    searcher_traverse_type_pack_id(self, ft.arg_types);
    self.flip();

    searcher_traverse_type_pack_id(self, ft.ret_types);

    self.is_within_function = old_value;

    false
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
    false
  }

  /// C++ `bool FreeTypeSearcher::visit(TypePackId tp, const FreeTypePack& ftp)`
  /// (Generalization.cpp:203-220).
  pub fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, ftp: &FreeTypePack) -> bool {
    if self.seen_with_current_polarity(tp) {
      return false;
    }

    if !subsumes(Some(self.scope), alias_opt(ftp.scope)) {
      return true;
    }

    // GeneralizationParams<TypePackId>& params = typePacks[tp]; ++params.useCount;
    self.type_packs.get_or_default(tp).use_count += 1;

    let is_within_function = self.is_within_function;
    let polarity = self.polarity;
    let params = self.type_packs.get_or_default(tp);

    if !is_within_function {
      params.found_outside_functions = true;
    }

    params.polarity |= polarity;

    true
  }
}
