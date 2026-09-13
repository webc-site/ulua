use crate::records::fixture::Fixture;

impl Fixture {
  /// C++ `void Fixture::limitStackSize(size_t size)` converts a desired stack
  /// budget into a `FInt::LuauStackGuardThreshold` scoped int via the
  /// platform-specific `getStackAddressSpaceSize()`. That helper is not part of
  /// the translated context, and no translated test drives this fixture method
  /// (the stack-guard test sets `FInt::LuauStackGuardThreshold` directly via
  /// `ScopedFastInt`). Unused scaffolding pending the platform helper.
  pub fn limit_stack_size(&mut self, _size: usize) {
    unreachable!(
      "Fixture::limit_stack_size requires platform getStackAddressSpaceSize (untranslated); no call site"
    );
  }
}
