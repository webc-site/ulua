use crate::records::{ir_function::IrFunction, store_location_hint::StoreLocationHint};

impl IrFunction {
  pub fn find_store_location_hint(&self, inst_idx: u32) -> Option<&StoreLocationHint> {
    self.store_location_hints.find(&inst_idx)
  }
}
