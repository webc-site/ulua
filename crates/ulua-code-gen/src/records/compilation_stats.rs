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

impl CompilationStats {
  pub const BYTECODE_SIZE_BYTES: usize = 0;
  pub const NATIVE_CODE_SIZE_BYTES: usize = 0;
  pub const NATIVE_DATA_SIZE_BYTES: usize = 0;
  pub const NATIVE_METADATA_SIZE_BYTES: usize = 0;
  pub const FUNCTIONS_TOTAL: u32 = 0;
  pub const FUNCTIONS_COMPILED: u32 = 0;
  pub const FUNCTIONS_BOUND: u32 = 0;
}
