#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum LuarequireNavigateResult {
  NavigateSuccess = 0,
  NavigateAmbiguous = 1,
  NavigateNotFound = 2,
}

pub use LuarequireNavigateResult as luarequire_NavigateResult;

// C 风格别名，供嵌入方按 C 枚举名使用。
impl LuarequireNavigateResult {
  pub const NAVIGATE_SUCCESS: Self = Self::NavigateSuccess;
  pub const NAVIGATE_AMBIGUOUS: Self = Self::NavigateAmbiguous;
  pub const NAVIGATE_NOT_FOUND: Self = Self::NavigateNotFound;
}
