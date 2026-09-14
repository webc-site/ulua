//! Node: `cxx:Record:Luau.Analysis:Analysis/src/ToString.cpp:49:find_cyclic_types`
//! Source: `Analysis/src/ToString.cpp:49-124` (hand-ported)
//!
//! C++ `struct FindCyclicTypes final : TypeVisitor` (anonymous namespace of
//! ToString.cpp). The virtual overrides live here as the
//! `GenericTypeVisitorTrait` impl (the per-method node files document this);
//! copy ctor and operator= are deleted in C++ — not derived here.
//!
use alloc::{collections::BTreeSet, string::String};
use core::{ffi::c_void, ptr::null};
use std::collections::HashSet;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    extern_type::ExternType,
    free_type::FreeType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    pending_expansion_type::PendingExpansionType,
    set::Set,
    table_type::TableType,
    type_visitor::TypeVisitor,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct FindCyclicTypes {
  pub base: TypeVisitor,
  pub exhaustive: bool,
  pub visited: Set<TypeId>,
  pub visited_packs: Set<TypePackId>,
  /// `std::set<TypeId>` — ordered (by pointer) iteration matters for
  /// deterministic cycle naming in `assignCycleNames`.
  pub cycles: BTreeSet<TypeId>,
  pub cycle_tps: BTreeSet<TypePackId>,
}

impl Default for FindCyclicTypes {
  fn default() -> Self {
    Self::new()
  }
}

impl FindCyclicTypes {
  /// C++ `FindCyclicTypes() : TypeVisitor("FindCyclicTypes", /* skipBoundTypes */ true)` (ToString.cpp:51).
  pub fn new() -> Self {
    Self {
      base: TypeVisitor::new(String::from("FindCyclicTypes"), true),
      exhaustive: false,
      visited: Set::new(null()),
      visited_packs: Set::new(null()),
      cycles: BTreeSet::new(),
      cycle_tps: BTreeSet::new(),
    }
  }

  /// `Luau::Set::insert(x) -> bool` (true when newly inserted).
  fn insert_visited(&mut self, ty: TypeId) -> bool {
    self.visited.insert(&ty)
  }

  fn insert_visited_pack(&mut self, tp: TypePackId) -> bool {
    self.visited_packs.insert(&tp)
  }
}

impl GenericTypeVisitorTrait for FindCyclicTypes {
  type Seen = HashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  /// ToString.cpp:65
  fn cycle_type_id(&mut self, ty: TypeId) {
    self.cycles.insert(ty);
  }

  /// ToString.cpp:70
  fn cycle_type_pack_id(&mut self, tp: TypePackId) {
    self.cycle_tps.insert(tp);
  }

  /// ToString.cpp:75
  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    self.insert_visited(ty)
  }

  /// ToString.cpp:80
  fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    self.insert_visited_pack(tp)
  }

  /// ToString.cpp:85
  fn visit_type_id_free_type(&mut self, ty: TypeId, ft: &FreeType) -> bool {
    if !self.insert_visited(ty) {
      return false;
    }
    LUAU_ASSERT!(!ft.lower_bound.is_null());
    LUAU_ASSERT!(!ft.upper_bound.is_null());
    self.traverse_type_id(ft.lower_bound);
    self.traverse_type_id(ft.upper_bound);
    false
  }

  /// ToString.cpp:96
  fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
    // C++ `FindCyclicTypes::visit(ty, TableType)` (ToString.cpp:96-98):
    //   if (!visited.insert(ty)) return false;
    // A *second* visit through the permanent `visited` set means this table was
    // reached again as a SIBLING (shared, acyclic DAG node) — NOT a cycle. C++
    // does NOT call `cycle()` here; it just stops descending. Genuine cycles are
    // caught upstream by the traverse's forgetting `seen` set (an ancestor
    // re-visit), which calls `cycle(ty)`. An earlier port mistakenly added a
    // `cycle_type_id(ty)` call here, which over-named shared-but-acyclic tables
    // (`(t1, t1) -> t1 where ...` instead of the inline form) — see
    // visit_type_visit_once / to_string_tostring_unsee_ttv_if_array. It also did
    // NOT fix the cyclic stack overflow (that was a dangling builtin_types
    // pointer in the test fixture).
    if !self.insert_visited(ty) {
      return false;
    }

    if ttv.name.is_some() || ttv.synthetic_name.is_some() {
      for &itp in ttv.instantiated_type_params.iter() {
        self.traverse_type_id(itp);
      }

      for &itp in ttv.instantiated_type_pack_params.iter() {
        self.traverse_type_pack_id(itp);
      }

      return self.exhaustive;
    }

    true
  }

  /// ToString.cpp:115
  fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }

  /// ToString.cpp:120
  fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    false
  }
}
