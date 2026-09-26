use core::ptr::NonNull;
pub type NativeProtoExecDataPtr = NonNull<u32>;

// C++ 源码中 NativeProtoExecDataPtr 是带自定义 deleter 的 std::unique_ptr；
// Rust 侧由本 crate 手动管理生命周期（SharedCodeAllocator 持有并释放）。
