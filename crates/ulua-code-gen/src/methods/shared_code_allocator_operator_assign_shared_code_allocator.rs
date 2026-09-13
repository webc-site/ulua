use crate::records::shared_code_allocator::SharedCodeAllocator;

impl SharedCodeAllocator {
  pub fn operator_assign(&mut self, _other: &SharedCodeAllocator) -> &mut SharedCodeAllocator {
    unreachable!("Deleted operator=");
  }
}
