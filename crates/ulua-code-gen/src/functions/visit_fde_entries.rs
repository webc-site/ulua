use core::ffi::{c_char, c_void};

/// cpp `visitFdeEntries`（`src/CodeBlockUnwind.cpp:85`）逐条遍历 unwind 数据中的 FDE。
/// # Safety
/// `pos` 必须指向以 4 字节长度为头的完整 FDE 序列，`cb` 须为有效的 C 回调。
pub unsafe fn visit_fde_entries(pos: *mut c_char, cb: unsafe extern "C" fn(*const c_void)) {
  // C++ Luau 用 *weak* 的 `__unw_add_dynamic_fde` 符号探测 Apple 的
  // libunwind：符号存在（Apple）时逐个注册每条 FDE 条目；
  // 符号缺失（Linux/其他平台，它们经 `__register_frame` 一次性注册整块）
  // 时直接把整块交给 `cb` 处理。
  // Rust 中 weak extern static 属于 *strong* 未定义引用，在 Linux 上会
  // 链接失败（"undefined symbol: __unw_add_dynamic_fde"），因此改为编译期
  // 探测平台——这正是原先 weak 符号存在性检查
  // 所代表的含义。

  // Safety: `# Safety` 契约保证 `cb` 为有效 C 回调、`pos` 指向完整 FDE 序列；
  // 非 Apple 直接把整块 pos 交给 cb（与 __register_frame 的注销同址）。
  #[cfg(not(target_vendor = "apple"))]
  unsafe {
    cb(pos as *const c_void);
  }

  // Safety（Apple 分支，分支内各窄块统一简记「依本分支契约」）：`# Safety` 契约保证
  // pos 指向以 4 字节长度为头的完整 FDE 序列；窄块以 read_unaligned 读非对齐长度/id
  // 以规避对齐 UB，以 part_length 界定 current_pos 前进、遇 0 终止，故读取不越出该序列。
  #[cfg(target_vendor = "apple")]
  {
    use core::ptr::read_unaligned;

    let mut current_pos = pos;
    loop {
      // FDE 头为非 4 字节对齐的流式数据，只能非对齐读取（cpp memcpy 同义）
      // 依本分支契约：读完整序列内 4 字节长度头。
      let part_length = unsafe { read_unaligned(current_pos as *const u32) };
      if part_length == 0 {
        break;
      }

      // 依本分支契约：part_length 非零，`.add(4)` 与 4 字节 id 读数仍界内。
      let part_id = unsafe { read_unaligned(current_pos.add(4) as *const u32) };
      if part_id != 0 {
        // 依本分支契约：`cb` 为有效回调，传条目起始地址（Copy 指针转换）。
        unsafe { cb(current_pos as *const c_void) };
      }

      // 依本分支契约：`part_length` 由契约的完整序列界定，指针前进不越界。
      current_pos = unsafe { current_pos.add(part_length as usize + 4) };
    }
  }
}
