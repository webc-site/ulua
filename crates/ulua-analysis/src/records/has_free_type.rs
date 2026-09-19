use alloc::string::String;
use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    extern_type::ExternType,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct HasFreeType {
  pub base: TypeOnceVisitor,
  pub result: bool,
}

impl HasFreeType {
  pub fn new() -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("TypeOnceVisitor"), true),
      result: false,
    }
  }

  pub fn has_free_type_has_free_type(&mut self) {
    *self = Self::new();
  }
}

impl Default for HasFreeType {
  fn default() -> Self {
    Self::new()
  }
}

impl GenericTypeVisitorTrait for HasFreeType {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    HasFreeType::visit_type_id(self, ty)
  }

  fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    HasFreeType::visit_type_pack_id(self, tp)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, ext: &ExternType) -> bool {
    self.visit_extern_type(ty, ext)
  }

  fn visit_type_id_free_type(&mut self, ty: TypeId, ft: &FreeType) -> bool {
    self.visit_free_type(ty, ft)
  }

  fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, ftp: &FreeTypePack) -> bool {
    self.visit_free_type_pack(tp, ftp)
  }
}

impl HasFreeType {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub(crate) fn visit_type_id(&mut self, ty: TypeId) -> bool {
    !(self.result || unsafe { (*ty).persistent })
  }

  pub fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    !self.result
  }

  pub fn visit_extern_type(&mut self, _ty: TypeId, _ext: &ExternType) -> bool {
    false
  }

  pub fn visit_free_type(&mut self, _ty: TypeId, _ft: &FreeType) -> bool {
    self.result = true;
    false
  }

  pub fn visit_free_type_pack(&mut self, _tp: TypePackId, _ftp: &FreeTypePack) -> bool {
    self.result = true;
    false
  }
}
