#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum LuarequireNavigateResult {
  NavigateSuccess = 0,
  NavigateAmbiguous = 1,
  NavigateNotFound = 2,
}

// C 风格常量别名，供嵌入方按 C 枚举名使用（类型本体用规范驼峰名）。
impl LuarequireNavigateResult {
  pub const NAVIGATE_SUCCESS: Self = Self::NavigateSuccess;
  pub const NAVIGATE_AMBIGUOUS: Self = Self::NavigateAmbiguous;
  pub const NAVIGATE_NOT_FOUND: Self = Self::NavigateNotFound;
}
