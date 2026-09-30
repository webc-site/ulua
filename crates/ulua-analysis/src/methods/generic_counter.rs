//! `generic_counter` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;

use ulua_common::{
  fint::LuauGenericCounterMaxSteps,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::polarity::Polarity,
  records::{
    extern_type::ExternType, generic_counter::GenericCounter, table_type::TableType,
    type_visitor::TypeVisitor,
  },
  type_aliases::type_id::TypeId,
};

impl GenericCounter {
  pub fn check_limits(&mut self) {
    self.steps += 1;
    // FInt::LuauGenericCounterMaxSteps access（cpp Generalization.cpp:21，默认 1500）
    if self.steps > LuauGenericCounterMaxSteps.get() {
      self.hit_limits = true;
    }
  }
}

impl GenericCounter {
  pub fn new(cached_types: *mut DenseHashSet<TypeId>) -> Self {
    Self {
      base: TypeVisitor::new(String::from("GenericCounter"), true),
      seen_counts: DenseHashMap::default(),
      cached_types,
      generics: DenseHashMap::default(),
      generic_packs: DenseHashMap::default(),
      polarity: Polarity::default(),
      steps: 0,
      hit_limits: false,
    }
  }
}

impl GenericCounter {
  pub fn visit_type_id(&mut self) -> bool {
    self.check_limits();
    !self.hit_limits
  }

  pub fn visit_type_id_function_type(&mut self) -> bool {
    self.check_limits();
    false
  }

  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _tt: &TableType) -> bool {
    self.check_limits();
    false
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
    false
  }

  pub fn visit_type_id_generic_type(&mut self) -> bool {
    // Mirrors the C++:
    // bool visit(TypeId ty, const GenericType&) override { ... }
    //
    // This Rust one-shot item only specifies the generic-type visit hook;
    // the TypeId is provided by the visitor dispatch into GenericCounter.
    self.visit_type_id()
  }

  pub fn visit_type_pack_id_generic_type_pack(&mut self) -> bool {
    false
  }
}
