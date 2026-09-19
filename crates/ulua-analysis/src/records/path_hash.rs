//! @interface-stub
use crate::{
  enums::{pack_field::PackField, type_field::TypeField},
  records::{
    generic_pack_mapping::GenericPackMapping, index::Index, pack_slice::PackSlice,
    property_type_path::Property, reduction::Reduction,
  },
  type_aliases::component::Component,
};

#[derive(Debug, Clone)]
pub struct PathHash;

impl PathHash {
  pub fn operator_property(&self, prop: &Property) -> usize {
    self.operator_call_7(prop)
  }
  pub fn operator_index(&self, idx: &Index) -> usize {
    self.operator_call_3(idx)
  }
  pub fn operator_type_field(&self, field: &TypeField) -> usize {
    self.operator_call_9(field)
  }
  pub fn operator_pack_field(&self, field: &PackField) -> usize {
    self.operator_call_4(field)
  }
  pub fn operator_pack_slice(&self, slice: &PackSlice) -> usize {
    self.operator_call_5(slice)
  }
  pub fn operator_reduction(&self, reduction: &Reduction) -> usize {
    self.operator_call_8(reduction)
  }
  pub fn operator_generic_pack_mapping(&self, mapping: &GenericPackMapping) -> usize {
    self.operator_call_2(mapping)
  }
  pub fn operator_component(&self, component: &Component) -> usize {
    match component {
      Component::Property(prop) => self.operator_property(prop),
      Component::Index(idx) => self.operator_index(idx),
      Component::TypeField(field) => self.operator_type_field(field),
      Component::PackField(field) => self.operator_pack_field(field),
      Component::PackSlice(slice) => self.operator_pack_slice(slice),
      Component::Reduction(reduction) => self.operator_reduction(reduction),
      Component::GenericPackMapping(mapping) => self.operator_generic_pack_mapping(mapping),
    }
  }
}
