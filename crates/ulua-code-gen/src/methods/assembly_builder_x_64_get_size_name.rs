use crate::{enums::size_x_64::SizeX64, records::assembly_builder_x_64::AssemblyBuilderX64};

impl AssemblyBuilderX64 {
  pub fn get_size_name(&self, size: SizeX64) -> &'static str {
    static SIZE_NAMES: &[&str] = &[
      "none", "byte", "word", "dword", "qword", "xmmword", "ymmword",
    ];

    let index = size as usize;
    if index >= SIZE_NAMES.len() {
      return "unknown";
    }
    SIZE_NAMES[index]
  }
}
