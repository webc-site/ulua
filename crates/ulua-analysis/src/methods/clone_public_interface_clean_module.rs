use core::ffi::c_void;
/// `GenericTypeFinder::visit` overrides are declared as inherent methods on the
/// record (Instantiation.h:84-114). The visitor driver `traverse` needs the
/// `GenericTypeVisitorTrait` surface, so wire the trait to those inherent
/// methods here (the dispatch points `ClonePublicInterface::clean` relies on).
use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::table_state::TableState,
  functions::{get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id},
  records::{
    blocked_type::BlockedType,
    clone_public_interface::ClonePublicInterface,
    extern_type::ExternType,
    free_type::FreeType,
    function_type::FunctionType,
    generic_type::GenericType,
    generic_type_finder::GenericTypeFinder,
    generic_type_pack::GenericTypePack,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    pending_expansion_type::PendingExpansionType,
    table_type::TableType,
    type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl GenericTypeVisitorTrait for GenericTypeFinder {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    GenericTypeFinder::visit_type_id(self, ty)
  }

  fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    GenericTypeFinder::visit_type_pack_id(self, tp)
  }

  fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    GenericTypeFinder::visit_type_id_function_type(self, ty, ftv)
  }

  fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
    GenericTypeFinder::visit_type_id_table_type(self, ty, ttv)
  }

  fn visit_type_id_generic_type(&mut self, ty: TypeId, gtv: &GenericType) -> bool {
    GenericTypeFinder::visit_type_id_generic_type(self, ty, gtv)
  }

  fn visit_type_pack_id_generic_type_pack(
    &mut self,
    tp: TypePackId,
    gtp: &GenericTypePack,
  ) -> bool {
    GenericTypeFinder::visit_type_pack_id_generic_type_pack(self, tp, gtp)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    GenericTypeFinder::visit_type_id_extern_type(self, ty, etv)
  }
}

impl ClonePublicInterface {
  /// `TypeId ClonePublicInterface::clean(TypeId ty)`.
  /// Reference: `Module.cpp:167-208`.
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let mut result = self.base.clone_type_id(ty);

    if let Some(ftv) = get_mutable_type_id::<FunctionType>(result) {
      if ftv.generics.is_empty() && ftv.generic_packs.is_empty() {
        let mut marker = GenericTypeFinder::new();
        marker.traverse_type_id(result);

        if !marker.found {
          ftv.has_no_free_or_generic_types = true;
        }
      }

      ftv.level = TypeLevel::new(0, 0);
    } else if let Some(ttv) = get_mutable_type_id::<TableType>(result) {
      ttv.level = TypeLevel::new(0, 0);
      if self.is_new_solver() {
        ttv.scope = null_mut();
        ttv.state = TableState::Sealed;
      }
    }

    if self.is_new_solver() {
      if get_type_id::<FreeType>(ty).is_some()
        || get_type_id::<BlockedType>(ty).is_some()
        || get_type_id::<PendingExpansionType>(ty).is_some()
      {
        self.internal_type_escaped = true;
        // SAFETY: builtin_types 指向会话期 BuiltinTypes，全局内建类型表。
        result = unsafe { (*self.builtin_types).error_type };
      } else if let Some(genericty) = get_mutable_type_id::<GenericType>(result) {
        genericty.scope = null_mut();
      }
    }

    result
  }
}
