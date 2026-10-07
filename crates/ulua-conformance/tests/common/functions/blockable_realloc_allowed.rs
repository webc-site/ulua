use core::sync::atomic::AtomicBool;

/// C++ 对应 `static bool blockableReallocAllowed = true;`。以 `AtomicBool` 承载：
/// 分配器回调（`blockable_realloc`）在 VM 线程读取，测试与 Lua 侧
/// `gc_set_block_allocations` 写入，`static mut` 在并行测试下即数据竞争。
pub static BLOCKABLE_REALLOC_ALLOWED: AtomicBool = AtomicBool::new(true);
