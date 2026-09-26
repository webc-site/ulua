/// cpp `LUAU_UNREACHABLE()`：向编译器声明此点不可达，以换取更优代码路径。
///
/// # Safety
/// 调用方必须保证该控制流点在运行期确实不可达；否则 `unreachable_unchecked`
/// 是 UB（优化器可据此删除其后的所有代码）。仅在「前一分支已由 `LUAU_ASSERT!`
/// 覆盖 / 枚举分支穷尽但编译器无法证明」处使用。
#[macro_export]
macro_rules! LUAU_UNREACHABLE {
  () => {
    // Safety: 前置条件即本宏的 `# Safety` —— 由调用方保证此处不可达。
    unsafe {
      core::hint::unreachable_unchecked();
    }
  };
}

pub use LUAU_UNREACHABLE;
