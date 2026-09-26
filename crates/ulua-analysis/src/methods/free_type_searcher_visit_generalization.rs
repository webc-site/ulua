use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    follow_type, follow_type_pack, get_type, get_type_pack, has_seen_visit_type::has_seen,
    subsumes_scope::subsumes, unsee_visit_type::unsee,
  },
  records::{
    extern_type::ExternType, free_type::FreeType, free_type_pack::FreeTypePack,
    free_type_searcher::FreeTypeSearcher, function_type::FunctionType,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, table_type::TableType, type_pack::TypePack, union_type::UnionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl FreeTypeSearcher {
  /// C++ `bool FreeTypeSearcher::visit(TypeId ty)` (Generalization.cpp:91-98) —
  /// the generic dispatch override. Guards re-traversal via the cached-type
  /// set and the polarity-aware seen set; returns `true` so the base
  /// `traverse` descends into children for the variants the searcher does not
  /// specialize.
  pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
    // Safety: self.cached_types 是 FreeTypeSearcher::new 从 generalize() 调用方（ConstraintSolver
    // 的 cache 集）接线注入的裸指针字段，指向 searcher 遍历全程存活、且此阶段仅被只读访问的对象
    // （TypeCacher 的插入发生在 searcher 遍历之后的独立阶段）；contains 取 &self，单线程串行读，
    // 重建共享引用不与任何可变借用重叠。
    if unsafe { (*self.cached_types).contains(&ty) }
      || self.seen_with_current_polarity(ty as *const ())
    {
      return false;
    }

    LUAU_ASSERT!(!ty.is_null());
    true
  }

  /// C++ `bool FreeTypeSearcher::visit(TypeId ty, const FreeType& ft)`
  /// (Generalization.cpp:100-117).
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, ft: &FreeType) -> bool {
    if !subsumes(self.scope, ft.scope) {
      return true;
    }

    // GeneralizationParams<TypeId>& params = types[ty]; ++params.useCount;
    self.types.get_or_default(ty).use_count += 1;

    // Safety: 同 visit_type_id——cached_types 是构造接线、在 searcher 遍历期内只读存活的
    // DenseHashSet<TypeId>，此处 contains 以 &self 单线程串行读，无重叠可变借用。
    if unsafe { (*self.cached_types).contains(&ty) }
      || self.seen_with_current_polarity(ty as *const ())
    {
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
    // Safety: 同 visit_type_id——cached_types 为构造接线、遍历期内只读存活的 DenseHashSet<TypeId>，
    // contains 以 &self 单线程串行读，不与其他借用产生别名冲突。
    if unsafe { (*self.cached_types).contains(&ty) }
      || self.seen_with_current_polarity(ty as *const ())
    {
      return false;
    }

    if matches!(tt.state, TableState::Free | TableState::Unsealed) && subsumes(self.scope, tt.scope)
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
        let p = self.polarity;
        self.polarity = Polarity::Negative;
        searcher_traverse_type_id(
          self,
          prop
            .write_ty
            .expect("is_write_only() 定义蕴含 write_ty 为 Some"),
        );
        self.polarity = p;
      } else if prop.is_shared() {
        let p = self.polarity;
        self.polarity = Polarity::Mixed;
        searcher_traverse_type_id(
          self,
          prop.read_ty.expect("is_shared() 定义蕴含 read_ty 为 Some"),
        );
        self.polarity = p;
      } else {
        searcher_traverse_type_id(
          self,
          prop
            .read_ty
            .expect("else 支为读写不一致态，Property 构造恒保证两侧皆 Some"),
        );
        let p = self.polarity;
        self.polarity = Polarity::Negative;
        searcher_traverse_type_id(
          self,
          prop
            .write_ty
            .expect("else 支为读写不一致态，Property 构造恒保证两侧皆 Some"),
        );
        self.polarity = p;
      }
    }

    if let Some(indexer) = &tt.indexer {
      // {[K]: V} is equivalent to get/set/iterate; K and V are mixed.
      let p = self.polarity;
      self.polarity = Polarity::Mixed;
      searcher_traverse_type_id(self, indexer.index_type);
      searcher_traverse_type_id(self, indexer.index_result_type);
      self.polarity = p;
    }

    false
  }
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
pub(crate) fn searcher_traverse_type_id(this: &mut FreeTypeSearcher, ty: TypeId) {
  let ty = follow_type::follow(ty);

  // C++ `if (hasSeen(seen, ty)) { cycle(ty); return; }`. `FreeTypeSearcher`
  // does not override `cycle`, so re-entry is a no-op return.
  if has_seen(&mut this.base.base.seen, ty as *const ()) {
    return;
  }

  searcher_dispatch_type_id(this, ty);

  unsee(&mut this.base.base.seen, ty as *const ());
}

/// Variant dispatch for `searcher_traverse_type_id` (the body of the C++
/// `traverse` switch). `ty` has already been followed and admitted past the
/// recursion-stack `seen` guard.
fn searcher_dispatch_type_id(this: &mut FreeTypeSearcher, ty: TypeId) {
  if let Some(ft) = get_type::get::<FreeType>(ty).as_ref() {
    if this.visit_type_id_free_type(ty, ft) {
      searcher_traverse_type_id(this, ft.lower_bound);
      searcher_traverse_type_id(this, ft.upper_bound);
    }
    return;
  }
  if let Some(tt) = get_type::get::<TableType>(ty).as_ref() {
    // visit(TypeId, const TableType&) performs its own polarity-aware
    // traversal and returns false.
    this.visit_type_id_table_type(ty, tt);
    return;
  }
  if let Some(ft) = get_type::get::<FunctionType>(ty).as_ref() {
    // visit(TypeId, const FunctionType&) (Generalization.cpp:179-196)
    // performs its own flipped traversal and returns false.
    if this.visit_type_id_function_type(ty, ft) {
      searcher_traverse_type_pack_id(this, ft.arg_types);
      searcher_traverse_type_pack_id(this, ft.ret_types);
    }
    return;
  }
  if let Some(et) = get_type::get::<ExternType>(ty).as_ref() {
    // visit(TypeId, const ExternType&) -> false (Generalization.cpp:198-201).
    this.visit_type_id_extern_type(ty, et);
    return;
  }

  // Variants the searcher does not specialize: run the base
  // `visit(TypeId)` guard, then descend through children.
  if !this.visit_type_id(ty) {
    return;
  }

  if let Some(ut) = get_type::get::<UnionType>(ty).as_ref() {
    for &opt in &ut.options {
      searcher_traverse_type_id(this, opt);
    }
  } else if let Some(it) = get_type::get::<IntersectionType>(ty).as_ref() {
    for &part in &it.parts {
      searcher_traverse_type_id(this, part);
    }
  } else if let Some(mt) = get_type::get::<MetatableType>(ty).as_ref() {
    searcher_traverse_type_id(this, mt.table);
    searcher_traverse_type_id(this, mt.metatable);
  } else if let Some(nt) = get_type::get::<NegationType>(ty).as_ref() {
    searcher_traverse_type_id(this, nt.ty);
  }
}

/// C++ `GenericTypeVisitor::traverse(TypePackId)` (VisitType.h:444-505) for the
/// `FreeTypeSearcher`. Same recursion-stack `seen` discipline as the type
/// traversal above.
pub(crate) fn searcher_traverse_type_pack_id(this: &mut FreeTypeSearcher, tp: TypePackId) {
  let tp = follow_type_pack::follow(tp);

  if has_seen(&mut this.base.base.seen, tp as *const ()) {
    return;
  }

  searcher_dispatch_type_pack_id(this, tp);

  unsee(&mut this.base.base.seen, tp as *const ());
}

/// Variant dispatch for `searcher_traverse_type_pack_id`.
fn searcher_dispatch_type_pack_id(this: &mut FreeTypeSearcher, tp: TypePackId) {
  if let Some(ftp) = get_type_pack::get::<FreeTypePack>(tp).as_ref() {
    this.visit_type_pack_id_free_type_pack(tp, ftp);
    return;
  }
  if let Some(pack) = get_type_pack::get::<TypePack>(tp).as_ref() {
    for &head in &pack.head {
      searcher_traverse_type_id(this, head);
    }
    if let Some(tail) = pack.tail {
      searcher_traverse_type_pack_id(this, tail);
    }
    return;
  }
  if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tp).as_ref() {
    searcher_traverse_type_id(this, vtp.ty);
  }
}

impl FreeTypeSearcher {
  /// C++ `bool FreeTypeSearcher::visit(TypeId ty, const FunctionType& ft)`
  /// (Generalization.cpp:179-196). Flips polarity across the parameter pack:
  /// arguments are traversed in inverted polarity (contravariant), return
  /// types in the current polarity (covariant). Marks the body as being
  /// within a function for the duration. Returns `false` because it performs
  /// its own traversal of the argument and return packs.
  pub fn visit_type_id_function_type(&mut self, ty: TypeId, ft: &FunctionType) -> bool {
    // Safety: 同 visit_type_id——cached_types 为构造接线、searcher 遍历期内只读存活的
    // DenseHashSet<TypeId>，contains 以 &self 单线程串行读，无重叠可变借用。
    if unsafe { (*self.cached_types).contains(&ty) }
      || self.seen_with_current_polarity(ty as *const ())
    {
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
    if self.seen_with_current_polarity(tp as *const ()) {
      return false;
    }

    if !subsumes(self.scope, ftp.scope) {
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
