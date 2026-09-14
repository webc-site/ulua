use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label},
};

impl AssemblyBuilderA64 {
  pub fn get_label_offset(&self, label: &Label) -> u32 {
    CODEGEN_ASSERT!(label.location != !0u32);
    label.location * 4
  }
}
