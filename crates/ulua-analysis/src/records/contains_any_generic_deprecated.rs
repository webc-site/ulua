//! C++ `struct ContainsAnyGeneric_DEPRECATED : TypeOnceVisitor`
//! (`Analysis/include/Luau/TypeUtils.h:404-420`, `Analysis/src/TypeUtils.cpp:947-981`).
//!
//! Faithful port: a `TypeOnceVisitor` (skipBoundTypes = true) that traverses a
//! type/type-pack and sets `found` if it encounters any `GenericType` /
//! `GenericTypePack`. Extern types are not descended into. The two static
//! `hasAnyGeneric` entry points construct the visitor, `traverse`, and return
//! `found`.
use alloc::string::String;
use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    extern_type::ExternType,
    generic_type::GenericType,
    generic_type_pack::GenericTypePack,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct ContainsAnyGenericDeprecated {
  pub base: TypeOnceVisitor,
  pub found: bool,
}

impl ContainsAnyGenericDeprecated {
  pub fn new() -> Self {
    Self {
      // C++: TypeOnceVisitor("ContainsAnyGeneric", /* skipBoundTypes */ true)
      base: TypeOnceVisitor::new(String::from("ContainsAnyGeneric"), true),
      found: false,
    }
  }

  pub fn has_any_generic(ty: TypeId) -> bool {
    let mut visitor = Self::new();
    visitor.traverse_type_id(ty);
    visitor.found
  }

  pub fn has_any_generic_type_pack_id(tp: TypePackId) -> bool {
    let mut visitor = Self::new();
    visitor.traverse_type_pack_id(tp);
    visitor.found
  }
}

impl Default for ContainsAnyGenericDeprecated {
  fn default() -> Self {
    Self::new()
  }
}

impl GenericTypeVisitorTrait for ContainsAnyGenericDeprecated {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  // C++ `bool visit(TypeId ty)`: `found = found || is<GenericType>(ty); return !found;`
  fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found
  }

  // C++ `bool visit(TypePackId ty)`: `found = found || is<GenericTypePack>(follow(ty)); return !found;`
  fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    !self.found
  }

  // The `is<GenericType>` / `is<GenericTypePack>` checks of the bare C++
  // `visit` overloads are expressed here via the per-member dispatch hooks the
  // traversal reaches for generic types/packs.
  fn visit_type_id_generic_type(&mut self, _ty: TypeId, _gtv: &GenericType) -> bool {
    self.found = true;
    false
  }

  fn visit_type_pack_id_generic_type_pack(
    &mut self,
    _tp: TypePackId,
    _gtp: &GenericTypePack,
  ) -> bool {
    self.found = true;
    false
  }

  // C++ `bool visit(TypeId ty, const ExternType&) { return false; }`
  fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}
