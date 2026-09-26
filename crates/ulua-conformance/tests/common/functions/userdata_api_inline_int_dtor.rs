use core::{ffi::c_void, sync::atomic::Ordering};

use crate::common::functions::userdata_api_dtor_hits::USERDATA_API_DTOR_HITS;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_api_inline_int_dtor(data: *mut c_void) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`data` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    USERDATA_API_DTOR_HITS.fetch_add(*(data as *const i32), Ordering::SeqCst);
  }
}
