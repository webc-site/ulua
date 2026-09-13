#[cfg(any())]
use crate::macros::codegen_assert::CODEGEN_ASSERT;
#[cfg(any())]
use crate::records::label::Label;

#[cfg(any())]
impl crate::records::assembly_builder_x_64::AssemblyBuilderX64 {
  pub(crate) fn get_label_offset(&self, label: &Label) -> u32 {
    CODEGEN_ASSERT!(label.location != !0u32);
    label.location
  }
}
