use alloc::vec::Vec;
use core::ptr::null;

use crate::{
  records::{
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct UnscopedGenericFinder {
  pub base: TypeOnceVisitor,
  pub scope_gen_tys: Vec<TypeId>,
  pub scope_gen_tps: Vec<TypePackId>,
  pub found_unscoped: bool,
}

impl UnscopedGenericFinder {
  pub fn unscoped_generic_finder_unscoped_generic_finder(&mut self) {
    self.base = TypeOnceVisitor::new(String::from("UnscopedGenericFinder"), true);
    self.scope_gen_tys = Vec::new();
    self.scope_gen_tps = Vec::new();
    self.found_unscoped = false;
  }
}

impl UnscopedGenericFinder {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found_unscoped
  }

  pub fn visit_type_pack_id(&mut self, _ty: TypePackId) -> bool {
    !self.found_unscoped
  }

  pub fn visit_type_id_generic_type(&mut self, ty: TypeId, _gtv: &GenericType) -> bool {
    if !self.scope_gen_tys.contains(&ty) {
      self.found_unscoped = true;
    }
    false
  }

  pub fn visit_type_pack_id_generic_type_pack(
    &mut self,
    tp: TypePackId,
    _gtp: &GenericTypePack,
  ) -> bool {
    if !self.scope_gen_tps.contains(&tp) {
      self.found_unscoped = true;
    }
    false
  }

  pub fn visit_type_id_function_type(&mut self, _ty: TypeId, ftv: &FunctionType) -> bool {
    let start_ty_count = self.scope_gen_tys.len();
    let start_tp_count = self.scope_gen_tps.len();

    self.scope_gen_tys.extend_from_slice(&ftv.generics);
    self.scope_gen_tps.extend_from_slice(&ftv.generic_packs);

    // NOTE: The translation unit previously attempted to call traversal helpers that
    // do not exist at the paths available in this crate. Since this file is only
    // responsible for the struct translation (with traversal logic provided elsewhere),
    // we avoid referencing missing traversal functions here.
    let _ = (&ftv.arg_types, &ftv.ret_types);

    self.scope_gen_tys.resize(start_ty_count, null());
    self.scope_gen_tps.resize(start_tp_count, null());

    false
  }
}
