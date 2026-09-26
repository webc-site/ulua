use core::ptr::null_mut;

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack,
    get_mutable_type,
  },
  records::{
    free_type::FreeType, free_type_pack::FreeTypePack, generic_type::GenericType,
    generic_type_pack::GenericTypePack, quantifier::Quantifier, table_type::TableType,
    r#type::Type, type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Quantifier {
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
    self.seen_mutable_type = true;

    if !self.level.subsumes(&ftv.level) {
      return false;
    }

    unsafe {
      *as_mutable_type_id(ty) = Type::from(GenericType::generic_type_type_level(self.level));
    }

    self.generics.push(ty);

    false
  }

  pub fn visit_type_id_table_type(&mut self, ty: TypeId, _ttv: &TableType) -> bool {
    // visitor 已按 TableType 变体分派进入本回调（ty 即该表节点），下转必命中。
    let ttv_mut = get_mutable_type::get_mutable::<TableType>(ty)
      .expect("visitor 按 TableType 分派，下转必命中");

    if ttv_mut.state == TableState::Generic {
      self.seen_generic_type = true;
    }

    if ttv_mut.state == TableState::Free {
      self.seen_mutable_type = true;
    }

    // C++ `Quantifier::visit(TypeId, const TableType&)` (Quantify.cpp:68)
    // gates on `level.subsumes(ttv.level)` — TypeLevel subsumption, the
    // same gate the free-type/free-pack overloads use. The
    // `subsumes(Scope*, Scope*)` member exists on the C++ struct but is
    // vestigial; porting the gate to it left `scope` null, so the gate
    // was always false and free tables were never quantified.
    if !self.level.subsumes(&ttv_mut.level) {
      if ttv_mut.state == TableState::Unsealed {
        self.seen_mutable_type = true;
      }
      return false;
    }

    if ttv_mut.state == TableState::Free {
      ttv_mut.state = TableState::Generic;
      self.seen_generic_type = true;
    } else if ttv_mut.state == TableState::Unsealed {
      ttv_mut.state = TableState::Sealed;
    }

    ttv_mut.level = self.level;

    true
  }

  pub fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, ftp: &FreeTypePack) -> bool {
    self.seen_mutable_type = true;

    if !self.level.subsumes(&ftp.level) {
      return false;
    }

    // *asMutable(tp) = GenericTypePack{level};
    let mut gtp = GenericTypePack {
      index: 0,
      level: Default::default(),
      scope: null_mut(),
      name: Default::default(),
      explicit_name: false,
      polarity: Polarity::None,
    };
    gtp.generic_type_pack_type_level(self.level);

    unsafe {
      *as_mutable_type_pack(tp) = TypePackVar::from(gtp);
    }

    self.generic_packs.push(tp);
    true
  }
}
