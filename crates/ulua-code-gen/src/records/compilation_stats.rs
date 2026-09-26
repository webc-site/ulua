#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CompilationStats {
  pub bytecode_size_bytes: usize,
  pub native_code_size_bytes: usize,
  pub native_data_size_bytes: usize,
  pub native_metadata_size_bytes: usize,
  pub functions_total: u32,
  pub functions_compiled: u32,
  pub functions_bound: u32,
}
