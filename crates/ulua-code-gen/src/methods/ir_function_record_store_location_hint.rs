use crate::records::{ir_function::IrFunction, store_location_hint::StoreLocationHint};

impl IrFunction {
  pub fn record_store_location_hint(&mut self, inst_idx: u32, hint: StoreLocationHint) {
    *self.store_location_hints.get_or_insert(inst_idx) = hint;
  }
}
