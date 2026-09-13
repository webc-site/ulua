#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum LuarequireWriteResult {
  WriteSuccess = 0,
  WriteBufferTooSmall = 1,
  WriteFailure = 2,
}

pub use LuarequireWriteResult as luarequire_WriteResult;

// C 风格别名，供嵌入方按 C 枚举名使用。
impl LuarequireWriteResult {
  pub const WRITE_SUCCESS: Self = Self::WriteSuccess;
  pub const WRITE_BUFFER_TOO_SMALL: Self = Self::WriteBufferTooSmall;
  pub const WRITE_FAILURE: Self = Self::WriteFailure;
}
