//! `FILE*` 的 RAII 包装。
//!
//! 对应 cpp `cpp/tests/Conformance.test.cpp:3551-3560` 的
//! `FILE* f = fopen(path, "w"); REQUIRE(f); lua_memorydump(L, f, nullptr); fclose(f);`。
//! 上游靠 `REQUIRE` 失败即中止整个用例（进程随后退出），本端口的断言失败会 unwind
//! 回 libtest，裸 `FILE*` 因此泄漏（连同其stdio 缓冲区），故把生命周期收进 `Drop`。

use core::{
  ffi::{c_char, c_void},
  ptr::NonNull,
};

use crate::common::functions::cstr::cstr;

unsafe extern "C" {
  fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
  fn fclose(file: *mut c_void) -> i32;
}

/// 只用于把 `FILE*` 交给 `lua_c_dump`（内部走 `fwrite`），不读取内容。
#[repr(transparent)]
pub struct CFileRef(NonNull<c_void>);

impl CFileRef {
  /// `fopen(path, "w")`：打不开返回 `None`，由调用方决定如何判负
  /// （cpp 的 `REQUIRE(f)` 等价物）。`path` 为 NUL 结尾字节串（`b"…\0"` 字面量）。
  pub fn create(path: &'static [u8]) -> Option<Self> {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`path` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    NonNull::new(unsafe { fopen(path.as_ptr().cast(), cstr(b"w\0")) }).map(Self)
  }

  /// 交给 C 侧写入的原指针；本类型仍持有所有权并在 `Drop` 时 `fclose`。
  pub fn as_ptr(&self) -> *mut c_void {
    self.0.as_ptr()
  }
}

impl Drop for CFileRef {
  fn drop(&mut self) {
    // 与 cpp 一致不检查 `fclose` 返回值：写入目标是 /dev/null（或 Windows 的 NUL），
    // 丢弃输出不是错误；真正的写入失败在上游同样不会被发现。
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      fclose(self.0.as_ptr());
    }
  }
}
