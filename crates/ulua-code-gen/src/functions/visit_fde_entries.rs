use core::ffi::c_void;

/// cpp `visitFdeEntries`（`src/CodeBlockUnwind.cpp:85`）逐条遍历 unwind 数据中的 FDE。
///
/// 遍历本身是纯 Rust 的切片游标（下标前进、`get` 有界读，取代旧 `*mut c_char pos`
/// 裸指针参数）；唯一的 C ABI 触点是 `cb`（`__register_frame`/`__deregister_frame`），
/// 其 unsafe 只落在回调调用行上。
///
/// `block` 为 unwind 数据所在代码块的完整可读字节（指针 + 长度对以切片表达）；
/// FDE 序列由 `finalize` 以 0 长度头终结，良构数据下切片界与终结符双重兜底，
/// 截断/畸形序列在下标越出切片时终止遍历（cpp 同形输入为越界 UB，此处取
/// 安全方向；DELIBERATE DEVIATION: CodeBlockUnwind.cpp visitFdeEntries 无长度上界）。
pub fn visit_fde_entries(block: &[u8], cb: unsafe extern "C" fn(*const c_void)) {
  // C++ Luau 用 *weak* 的 `__unw_add_dynamic_fde` 符号探测 Apple 的
  // libunwind：符号存在（Apple）时逐个注册每条 FDE 条目；
  // 符号缺失（Linux/其他平台，它们经 `__register_frame` 一次性注册整块）
  // 时直接把整块交给 `cb` 处理。
  // Rust 中 weak extern static 属于 *strong* 未定义引用，在 Linux 上会
  // 链接失败（"undefined symbol: __unw_add_dynamic_fde"），因此改为编译期
  // 探测平台——这正是原先 weak 符号存在性检查所代表的含义。

  // Safety: `cb` 为调用点提供的有效 libunwind C 回调；实参为切片首地址，
  // 即整块 unwind 数据的起始指针（与 __register_frame/__deregister_frame 的注销同址契约）。
  #[cfg(not(target_vendor = "apple"))]
  unsafe {
    cb(block.as_ptr().cast());
  }

  #[cfg(target_vendor = "apple")]
  {
    // FDE 头为非 4 字节对齐的流式数据；cpp 用 memcpy 取本机字序 u32，
    // 此处切片定长数组 + from_ne_bytes 与之逐位等价（codegen 目标恒为小端）。
    const HEADER_LEN: usize = 8;
    let mut pos = 0usize;
    // 切片有界读即循环终止条件：越出切片 → `get` 返回 `None` → `while let` 收束
    // （良构序列必由 0 长度头先行终结，与 cpp 同形）。
    while let Some(header) = block.get(pos..pos + HEADER_LEN) {
      let part_length = u32::from_ne_bytes(header[0..4].try_into().unwrap()) as usize;
      if part_length == 0 {
        break;
      }

      let part_id = u32::from_ne_bytes(header[4..8].try_into().unwrap());
      if part_id != 0 {
        // Safety: `cb` 为有效回调；实参是本条 FDE 头部在同一存活映射块内的起始地址
        // （pos 已由切片界约束，条目体在契约的完整序列内）。
        unsafe { cb(block.as_ptr().add(pos).cast()) };
      }

      // 游标前进：长度头 4 字节 + part_length 的条目体；若越出切片，下一轮 get
      // 返回 None 终止遍历（良构序列必由 0 长度头先行终结，与 cpp 同形）。
      pos += part_length + 4;
    }
  }
}
