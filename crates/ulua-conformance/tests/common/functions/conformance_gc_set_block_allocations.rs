use core::{ffi::c_int, sync::atomic::Ordering};

use ulua_vm::{functions::lua_l_checkboolean::lua_l_checkboolean, records::lua_state::LuaState};

use crate::common::functions::blockable_realloc_allowed::BLOCKABLE_REALLOC_ALLOWED;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_gc_set_block_allocations(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    BLOCKABLE_REALLOC_ALLOWED.store(lua_l_checkboolean(l, 1) == 0, Ordering::Relaxed);

    0
  }
}
