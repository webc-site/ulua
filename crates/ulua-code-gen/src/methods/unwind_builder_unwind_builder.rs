use crate::records::unwind_builder::UnwindBuilder;

impl Drop for UnwindBuilder {
  fn drop(&mut self) {
    // virtual ~UnwindBuilder() = default;
  }
}
