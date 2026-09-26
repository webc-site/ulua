#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum LuarequireWriteResult {
  WriteSuccess = 0,
  WriteBufferTooSmall = 1,
  WriteFailure = 2,
}

// C 风格常量别名，供嵌入方按 C 枚举名使用（类型本体用规范驼峰名）。
impl LuarequireWriteResult {
  pub const WRITE_SUCCESS: Self = Self::WriteSuccess;
  pub const WRITE_BUFFER_TOO_SMALL: Self = Self::WriteBufferTooSmall;
  pub const WRITE_FAILURE: Self = Self::WriteFailure;
}
