use core::ffi::c_void;

use crate::macros::codegen_assert::CODEGEN_ASSERT;
#[cfg(not(target_os = "windows"))]
use crate::{
  functions::visit_fde_entries::visit_fde_entries,
  records::unwind_builder_dwarf_2::UnwindBuilderDwarf2,
};

#[cfg(not(target_os = "windows"))]
unsafe extern "C" {
  fn __register_frame(begin: *const c_void);
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
use core::ffi::c_char;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
unsafe extern "C" {
  fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

#[cfg(target_os = "windows")]
unsafe extern "system" {
  fn RtlAddFunctionTable(function_table: *mut c_void, entry_count: u32, base_address: usize)
  -> i32;
}

/// Windows 面：定稿 SEH unwind 信息并注册函数表。返回 `Some`（builder 起始偏移, unwind 尺寸）；
/// 函数表注册失败时对齐 cpp 返回 `None`（调用方据此回 null 且不写 `begin_offset`）。
///
/// # Safety
/// `context` 必须指向存活的 `UnwindBuilderWin`（caller 即 codegen 上层按 cpp 约定保证）；
/// `block` 指向 `block_size` 字节可读写代码块，`finalize` 写入不越过该长度。
#[cfg(target_os = "windows")]
unsafe fn finalize_and_register_win_unwind(
  context: *mut c_void,
  block: *mut u8,
  block_size: usize,
) -> Option<(usize, usize)> {
  const K_CODE_ALIGNMENT: usize = 32;

  // Safety: (b) 最小封装契约——`context` 依调用方约定即存活的 `UnwindBuilderWin` 实例
  // （cfg 分支与 builder 实际类型一致），向下转型有效；所得 `&mut` 独占用于本助手，单线程无并存别名。
  let unwind =
    unsafe { &mut *(context.cast::<crate::records::unwind_builder_win::UnwindBuilderWin>()) };

  let unwind_size =
    (unwind.get_unwind_info_size(block_size) + (K_CODE_ALIGNMENT - 1)) & !(K_CODE_ALIGNMENT - 1);

  CODEGEN_ASSERT!(block_size >= unwind_size);

  let function_count = unwind.finalize(block.cast(), unwind_size, block.cast(), block_size);
  let builder_begin_offset = unwind.get_begin_offset();

  // Safety: (c) 真 FFI 边界——`RtlAddFunctionTable` 按 Win64 SEH 约定接收落在
  // `[block, block+block_size)` 内的函数表与代码基址（上方 CODEGEN_ASSERT 复核长度）。
  if unsafe { RtlAddFunctionTable(block.cast(), function_count as u32, block as usize) } == 0 {
    CODEGEN_ASSERT!(false);
    return None;
  }

  Some((builder_begin_offset, unwind_size))
}

/// 非 Windows 面：定稿 DWARF CIE/FDE 并逐 FDE 调用 `__register_frame`。返回（builder 起始偏移, unwind 尺寸）。
///
/// # Safety
/// `context` 必须指向存活的 `UnwindBuilderDwarf2`；`block` 指向 `block_size` 字节可读写
/// 代码块，`finalize` 写入不越过该长度。
#[cfg(not(target_os = "windows"))]
unsafe fn finalize_and_register_dwarf_unwind(
  context: *mut c_void,
  block: *mut u8,
  block_size: usize,
) -> (usize, usize) {
  const K_CODE_ALIGNMENT: usize = 32;

  // Safety: (b) 最小封装契约——`context` 依调用方约定即存活的 `UnwindBuilderDwarf2` 实例
  // （cfg 分支与 builder 实际类型一致），向下转型有效；所得 `&mut` 独占用于本助手，单线程无并存别名。
  let unwind = unsafe { &mut *(context.cast::<UnwindBuilderDwarf2>()) };

  let unwind_size =
    (unwind.get_unwind_info_size(block_size) + (K_CODE_ALIGNMENT - 1)) & !(K_CODE_ALIGNMENT - 1);

  CODEGEN_ASSERT!(block_size >= unwind_size);

  // Safety: (b) `finalize` 按其契约把 unwind 表写入 `[block, block+block_size)` 可写代码块，
  // 长度由上方 CODEGEN_ASSERT 复核。
  unsafe { unwind.finalize(block.cast(), unwind_size, block.cast(), block_size) };

  let builder_begin_offset = unwind.get_begin_offset();

  // Safety: (c) 真 FFI 边界——`visit_fde_entries` 以 `__register_frame`（libunwind C ABI）
  // 逐个注册 finalize 刚写入 `[block, block+block_size)` 的 FDE 头。
  unsafe { visit_fde_entries(block.cast(), __register_frame) };

  (builder_begin_offset, unwind_size)
}

/// # Safety
/// 由 codegen 上层按 cpp `createBlockUnwindInfo` 约定调用：`context` 是指向本平台具体 unwind
/// 构建器（Dwarf2/Win，与 `cfg` 分支一致）的存活对象指针；`block` 指向 `block_size` 字节可读写的
/// 代码块，`finalize` 写入不越过该长度；`begin_offset` 为存活 `&mut usize`。
pub unsafe extern "C-unwind" fn create_block_unwind_info(
  context: *mut c_void,
  block: *mut u8,
  block_size: usize,
  begin_offset: &mut usize,
) -> *mut c_void {
  // 平台差异收敛进下方两个窄 unsafe 助手；此处仅做契约透传与汇合。
  #[cfg(target_os = "windows")]
  // Safety: 满足 `create_block_unwind_info` 函数头契约——`context` 与 windows 分支的
  // `UnwindBuilderWin` 一致，读写均在 `[block, block+block_size)` 内。
  let Some((builder_begin_offset, unwind_size)) =
    (unsafe { finalize_and_register_win_unwind(context, block, block_size) })
  else {
    return core::ptr::null_mut();
  };

  #[cfg(not(target_os = "windows"))]
  // Safety: 同上——非 windows 分支 `context` 即 `UnwindBuilderDwarf2`，读写界内由助手内
  // CODEGEN_ASSERT 复核。
  let (builder_begin_offset, unwind_size) =
    unsafe { finalize_and_register_dwarf_unwind(context, block, block_size) };

  #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
  register_macos_a64_unwind_sections();

  *begin_offset = unwind_size + builder_begin_offset;
  block.cast()
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn register_macos_a64_unwind_sections() {
  use core::mem::transmute;
  use std::sync::OnceLock;

  use crate::{
    functions::find_dynamic_unwind_sections::find_dynamic_unwind_sections,
    records::unw_dynamic_unwind_sections_t::unw_dynamic_unwind_sections_t,
    type_aliases::unw_add_find_dynamic_unwind_sections_t::UnwAddFindDynamicUnwindSectionsT,
  };

  unsafe extern "C-unwind" fn find_dynamic_unwind_sections_thunk(
    addr: usize,
    info: *mut unw_dynamic_unwind_sections_t,
  ) -> i32 {
    if info.is_null() {
      return 0;
    }

    // Safety: 上一分支已判定 `info` 非 null;它由 libunwind 调用方提供,在本次回调期间为指向
    // `unw_dynamic_unwind_sections_t` 的对齐存活对象,重建 `&mut` 独占且借用随调用结束。
    find_dynamic_unwind_sections(addr, unsafe { &mut *info })
  }

  static REGISTER_RESULT: OnceLock<i32> = OnceLock::new();

  let result = REGISTER_RESULT.get_or_init(|| {
    // 本闭包只做三件 unsafe 事——`dlsym` 查符号、把返回的裸指针 `transmute` 为签名匹配的
    // `Option<extern fn>`、以及(符号存在时)调用注册函数传入合法 thunk;以下逐处注释其前置条件。
    const RTLD_DEFAULT: *mut c_void = -2_isize as *mut c_void;
    // review.md §10 常量形态：`&[u8]` NUL 结尾字节串，收口点仅 `.as_ptr().cast()`
    let symbol = b"__unw_add_find_dynamic_unwind_sections\0";

    // Safety: `dlsym` 返回 `*mut c_void`(符号地址或 null);借函数指针的 niche 优化 `transmute` 成
    // `Option<UnwAddFindDynamicUnwindSectionsT>`,该类型与 macOS 该私有符号的真实 C 签名逐字节一致,
    // 故转位模式合法:null → None,非 null → Some(有效 fn 地址)。
    let add_find_dynamic_unwind_sections: UnwAddFindDynamicUnwindSectionsT =
      unsafe { transmute(dlsym(RTLD_DEFAULT, symbol.as_ptr().cast())) };

    match add_find_dynamic_unwind_sections {
      // Safety: 仅在 `dlsym` 取到符号(Some)时调用真实注册函数,并传入合法的
      // `extern "C-unwind"` thunk 指针,符合 `__unw_add_find_dynamic_unwind_sections` 的入参约定。
      Some(register) => unsafe { register(Some(find_dynamic_unwind_sections_thunk)) },
      None => 0,
    }
  });

  CODEGEN_ASSERT!(*result == 0);
}
