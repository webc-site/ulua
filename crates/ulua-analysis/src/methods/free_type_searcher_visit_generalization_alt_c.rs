use core::ffi::c_void;

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id, has_seen_visit_type::has_seen,
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
  /// C++ `bool FreeTypeSearcher::visit(TypeId ty, const TableType& tt)`
  /// (Generalization.cpp:119-177).
  pub fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
    if unsafe { (*self.cached_types).contains(&ty) }
      || self.seen_with_current_polarity(ty as *const c_void)
    {
      return false;
    }

    if (tt.state == TableState::Free || tt.state == TableState::Unsealed)
      && subsumes(self.scope, tt.scope)
    {
      self.unsealed_tables.insert(ty);
    }

    for prop in tt.props.values() {
      if prop.is_read_only() {
        searcher_traverse_type_id(self, prop.read_ty.unwrap());
      } else if prop.is_write_only() {
        let p = self.polarity;
        self.polarity = Polarity::Negative;
        searcher_traverse_type_id(self, prop.write_ty.unwrap());
        self.polarity = p;
      } else if prop.is_shared() {
        let p = self.polarity;
        self.polarity = Polarity::Mixed;
        searcher_traverse_type_id(self, prop.read_ty.unwrap());
        self.polarity = p;
      } else {
        searcher_traverse_type_id(self, prop.read_ty.unwrap());
        let p = self.polarity;
        self.polarity = Polarity::Negative;
        searcher_traverse_type_id(self, prop.write_ty.unwrap());
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
  let ty = follow_type_id(ty);

  // C++ `if (hasSeen(seen, ty)) { cycle(ty); return; }`. `FreeTypeSearcher`
  // does not override `cycle`, so re-entry is a no-op return.
  if has_seen(&mut this.base.base.seen, ty as *const c_void) {
    return;
  }

  searcher_dispatch_type_id(this, ty);

  unsee(&mut this.base.base.seen, ty as *const c_void);
}

/// Variant dispatch for `searcher_traverse_type_id` (the body of the C++
/// `traverse` switch). `ty` has already been followed and admitted past the
/// recursion-stack `seen` guard.
fn searcher_dispatch_type_id(this: &mut FreeTypeSearcher, ty: TypeId) {
  if let Some(ft) = get_type_id::<FreeType>(ty).as_ref() {
    if this.visit_type_id_free_type(ty, ft) {
      searcher_traverse_type_id(this, ft.lower_bound);
      searcher_traverse_type_id(this, ft.upper_bound);
    }
    return;
  }
  if let Some(tt) = get_type_id::<TableType>(ty).as_ref() {
    // visit(TypeId, const TableType&) performs its own polarity-aware
    // traversal and returns false.
    this.visit_type_id_table_type(ty, tt);
    return;
  }
  if let Some(ft) = get_type_id::<FunctionType>(ty).as_ref() {
    // visit(TypeId, const FunctionType&) (Generalization.cpp:179-196)
    // performs its own flipped traversal and returns false.
    if this.visit_type_id_function_type(ty, ft) {
      searcher_traverse_type_pack_id(this, ft.arg_types);
      searcher_traverse_type_pack_id(this, ft.ret_types);
    }
    return;
  }
  if let Some(et) = get_type_id::<ExternType>(ty).as_ref() {
    // visit(TypeId, const ExternType&) -> false (Generalization.cpp:198-201).
    this.visit_type_id_extern_type(ty, et);
    return;
  }

  // Variants the searcher does not specialize: run the base
  // `visit(TypeId)` guard, then descend through children.
  if !this.visit_type_id(ty) {
    return;
  }

  if let Some(ut) = get_type_id::<UnionType>(ty).as_ref() {
    for &opt in &ut.options {
      searcher_traverse_type_id(this, opt);
    }
  } else if let Some(it) = get_type_id::<IntersectionType>(ty).as_ref() {
    for &part in &it.parts {
      searcher_traverse_type_id(this, part);
    }
  } else if let Some(mt) = get_type_id::<MetatableType>(ty).as_ref() {
    searcher_traverse_type_id(this, mt.table);
    searcher_traverse_type_id(this, mt.metatable);
  } else if let Some(nt) = get_type_id::<NegationType>(ty).as_ref() {
    searcher_traverse_type_id(this, nt.ty);
  }
}

/// C++ `GenericTypeVisitor::traverse(TypePackId)` (VisitType.h:444-505) for the
/// `FreeTypeSearcher`. Same recursion-stack `seen` discipline as the type
/// traversal above.
pub(crate) fn searcher_traverse_type_pack_id(this: &mut FreeTypeSearcher, tp: TypePackId) {
  let tp = unsafe { follow_type_pack_id(tp) };

  if has_seen(&mut this.base.base.seen, tp as *const c_void) {
    return;
  }

  searcher_dispatch_type_pack_id(this, tp);

  unsee(&mut this.base.base.seen, tp as *const c_void);
}

/// Variant dispatch for `searcher_traverse_type_pack_id`.
fn searcher_dispatch_type_pack_id(this: &mut FreeTypeSearcher, tp: TypePackId) {
  if let Some(ftp) = get_type_pack_id::<FreeTypePack>(tp).as_ref() {
    this.visit_type_pack_id_free_type_pack(tp, ftp);
    return;
  }
  if let Some(pack) = get_type_pack_id::<TypePack>(tp).as_ref() {
    for &head in &pack.head {
      searcher_traverse_type_id(this, head);
    }
    if let Some(tail) = pack.tail {
      searcher_traverse_type_pack_id(this, tail);
    }
    return;
  }
  if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tp).as_ref() {
    searcher_traverse_type_id(this, vtp.ty);
  }
}
