use core::ffi::c_void;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::get_mutable_type::get_mutable,
  records::{
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_type::FunctionType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    quantifier::Quantifier,
    table_type::TableType,
    type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

// C++ `struct Quantifier final : TypeOnceVisitor` (Quantify.cpp:15). The
// virtual `visit(...)` overrides live as the `GenericTypeVisitorTrait` impl
// (the `IndexCollector`/`FindCyclicTypes` precedents) so `traverse` dispatches
// into them; the bodies delegate to the inherent methods declared on the
// sibling `quantifier_visit_quantify*` files.
impl GenericTypeVisitorTrait for Quantifier {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
    Quantifier::visit_type_id_free_type(self, ty, ftv)
  }

  fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
    Quantifier::visit_type_id_table_type(self, ty, ttv)
  }

  fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, ftp: &FreeTypePack) -> bool {
    Quantifier::visit_type_pack_id_free_type_pack(self, tp, ftp)
  }
}

pub fn quantify(ty: TypeId, level: TypeLevel) {
  let mut q = Quantifier::new(level);
  q.traverse_type_id(ty);

  // C++: FunctionType* ftv = getMutable<FunctionType>(ty); LUAU_ASSERT(ftv);
  let ftv = get_mutable::<FunctionType>(ty);
  LUAU_ASSERT!(ftv.is_some());
  if let Some(ftv) = ftv {
    ftv.generics.extend(q.generics.iter().copied());
    ftv.generic_packs.extend(q.generic_packs.iter().copied());
  }
}
