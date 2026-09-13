use crate::records::shared_code_allocator::SharedCodeAllocator;

impl SharedCodeAllocator {
  pub fn operator_assign_mut(
    &mut self,
    _other: &mut SharedCodeAllocator,
  ) -> &mut SharedCodeAllocator {
    unreachable!("Deleted operator=");
  }
}
