use crate::records::ir_builder_fixture::IrBuilderFixture;

impl IrBuilderFixture {
  /// Dead duplicate skeleton ctor: the canonical constructor is
  /// `IrBuilderFixture::new()` / `Default` in
  /// `crates/luau-unit-test/src/records/ir_builder_fixture.rs`. This `&mut self`
  /// no-op variant has no call site.
  pub fn ir_builder_fixture(&mut self) {
    unreachable!(
      "canonical IrBuilderFixture::new lives in crates/luau-unit-test/src/records/ir_builder_fixture.rs; this skeleton node is unused"
    );
  }
}
