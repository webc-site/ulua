//! Source: `tests/CodeAllocator.test.cpp` —— 行为钉：输入→输出与 cpp oracle 一致。
//!
//! unsafe 收口说明：
//! - `CodeAllocator::allocate` 的 C++ 同款 `(指针, 长度)` 形参由本文件的
//!   [`allocate`] / [`allocate_u32_code`] 安全封装（空切片按 oracle 形态传 null 配 0）；
//! - 指针偏移断言一律改为地址（`usize`）算术，不再做指针 `add/sub`；
//! - 合理保留的 `unsafe` 仅有三处形态：allocate 封装内的 C ABI 入口调用、
//!   [`entry`] 将 JIT 代码地址装载为函数指针（对应 cpp `reinterpret_cast<FunctionType>`）、
//!   Dwarf2 `finalize` 的裸缓冲封装 [`finalize_dwarf2`]。

#[cfg(target_arch = "aarch64")]
use core::mem::size_of;
use core::{
  ffi::c_void,
  mem::transmute_copy,
  ptr::{null, null_mut},
};
extern crate alloc;

use ulua_code_gen::records::{
  code_allocation_data::CodeAllocationData, code_allocator::CodeAllocator,
  unwind_builder_dwarf_2::UnwindBuilderDwarf2,
};

/// cpp oracle 的 `(ptr, size)` 形参形态：空切片传 null 配 0，非空传切片首址。
fn ptr_len<T>(data: &[T]) -> (*const T, usize) {
  if data.is_empty() {
    (null(), 0)
  } else {
    (data.as_ptr(), data.len())
  }
}

/// `CodeAllocator::allocate`（u8 码流）的安全封装，测试内唯一 allocate 触点之一。
fn allocate(allocator: &mut CodeAllocator, data: &[u8], code: &[u8]) -> CodeAllocationData {
  let (data_ptr, data_len) = ptr_len(data);
  let (code_ptr, code_len) = ptr_len(code);
  // Safety: 两对指针/长度均由本帧存活切片物化（或 null 配 0），allocator 独占可变借用。
  unsafe { allocator.allocate(data_ptr, data_len, code_ptr, code_len) }
}

/// `CodeAllocator::allocate`（A64 的 u32 码流按字节喂入）的安全封装。
#[cfg(target_arch = "aarch64")]
fn allocate_u32_code(
  allocator: &mut CodeAllocator,
  data: &[u8],
  code: &[u32],
) -> CodeAllocationData {
  let (data_ptr, data_len) = ptr_len(data);
  let (code_ptr, code_words) = ptr_len(code);
  // Safety: 同 allocate；u32 码数组按 callee 契约以 (首址, 字数×4) 的字节流读取，切片本帧存活。
  unsafe {
    allocator.allocate(
      data_ptr,
      data_len,
      code_ptr.cast(),
      code_words * size_of::<u32>(),
    )
  }
}

/// C ABI 回调 context 的类型擦除句柄（cpp `static_cast<void*>(&x)` 同形态）。
fn void_ptr<T>(value: &mut T) -> *mut c_void {
  value as *mut T as *mut c_void
}

/// 将 `base + offset_bytes` 处的代码入口装载为 `extern "C-unwind" fn`
/// （对应 cpp `reinterpret_cast<FunctionType>(codeStart + offset)` 的行为钉）。
///
/// 测试基建契约：只允许传入 `CodeAllocator::allocate` 刚返回、且 codegen 已写入
/// 与 `F` 签名匹配的机器码的可执行地址。
fn entry<F: Copy>(base: *mut u8, offset_bytes: usize) -> F {
  let addr = base as usize + offset_bytes;
  // Safety: 按上述契约 addr 指向已写入的可执行入口；C ABI 函数指针与裸地址同布局，
  // `transmute_copy` 在 `F: Copy` 且尺寸一致（均为机器字宽指针）前提下安全。
  unsafe { transmute_copy::<usize, F>(&addr) }
}

/// Dwarf2 `finalize` 的裸缓冲形参封装（unsafe 收口到本函数一处）。
fn finalize_dwarf2(unwind: &UnwindBuilderDwarf2, data: &mut [u8]) {
  // Safety: target 由存活可变切片物化且长度即 get_unwind_info_size(0)（调用方构造），
  // offset/func_address/block_size 取 (0, null, 0) 与 callee 的空可选参数契约一致。
  unsafe { unwind.finalize(data.as_mut_ptr().cast(), 0, null_mut(), 0) };
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_code_allocation() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  const K_CODE_ALIGNMENT: usize = 32;

  let _protect_data = ScopedFastFlag::new(&fflag::LuauCodegenProtectData, false);

  let block_size = 1024 * 1024;
  let max_total_size = 1024 * 1024;
  let mut allocator = CodeAllocator::default();
  allocator.code_allocator_usize_usize(block_size, max_total_size);

  let code = [0_u8; 128];

  let result1 = allocate(&mut allocator, &[], &code);
  assert!(!result1.start.is_null());
  assert_eq!(result1.size, 128);
  assert!(!result1.code_start.is_null());
  assert_eq!(result1.code_start as usize, result1.start as usize);

  let data = [0_u8; 8];

  let result2 = allocate(&mut allocator, &data, &code);
  assert!(!result2.start.is_null());
  assert_eq!(result2.size, K_CODE_ALIGNMENT + 128);
  assert!(!result2.code_start.is_null());
  assert_eq!(
    result2.start as usize + K_CODE_ALIGNMENT,
    result2.code_start as usize
  );

  allocator.deallocate(result1);
  allocator.deallocate(result2);
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_code_allocation_callbacks() {
  use ulua_unit_test::{
    functions::allocation_callback_code_allocator_test::allocation_callback_code_allocator_test,
    records::allocation_data::AllocationData,
  };

  let block_size = 1024 * 1024;
  let max_total_size = 1024 * 1024;
  let mut allocation_data = AllocationData::default();

  {
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize_allocation_callback_void(
      block_size,
      max_total_size,
      Some(allocation_callback_code_allocator_test),
      void_ptr(&mut allocation_data),
    );

    let code = [0_u8; 128];

    let result = allocate(&mut allocator, &[], &code);
    assert!(!result.start.is_null());
    assert_eq!(allocation_data.bytes_allocated, block_size);
    assert_eq!(allocation_data.bytes_freed, 0);

    allocator.deallocate(result);
  }

  assert_eq!(allocation_data.bytes_allocated, block_size);
  assert_eq!(allocation_data.bytes_freed, block_size);
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_code_allocation_failure() {
  let block_size = 3000;
  let max_total_size = 7000;
  let mut allocator = CodeAllocator::default();
  allocator.code_allocator_usize_usize(block_size, max_total_size);

  let mut code = vec![0_u8; 4000];

  let result1 = allocate(&mut allocator, &[], &code);
  assert!(result1.start.is_null());

  code.resize(2000, 0);
  let result2 = allocate(&mut allocator, &[], &code);
  assert!(!result2.start.is_null());
  let result3 = allocate(&mut allocator, &[], &code);
  assert!(!result3.start.is_null());
  let result4 = allocate(&mut allocator, &[], &code);
  assert!(result4.start.is_null());

  allocator.deallocate(result2);
  allocator.deallocate(result3);
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_code_allocation_protect_data() {
  use ulua_common::fflag;
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  let _protect_data = ScopedFastFlag::new(&fflag::LuauCodegenProtectData, true);

  let block_size = 1024 * 1024;
  let max_total_size = 1024 * 1024;
  let mut allocator = CodeAllocator::default();
  allocator.code_allocator_usize_usize(block_size, max_total_size);

  let code = [0_u8; 128];
  let result1 = allocate(&mut allocator, &[], &code);
  assert!(!result1.start.is_null());
  assert_eq!(result1.size, 128);
  assert!(!result1.code_start.is_null());
  assert_eq!(result1.code_start as usize, result1.start as usize);

  let data = [0_u8; 8];
  let result2 = allocate(&mut allocator, &data, &code);
  assert!(!result2.start.is_null());
  assert_eq!(
    result2.size,
    CodeAllocator::align_to_page_size(data.len()) + code.len()
  );
  assert!(!result2.code_start.is_null());
  assert_eq!(
    result2.code_start as usize,
    CodeAllocator::align_to_page_size(result2.code_start as usize)
  );
  assert!(result2.code_start as usize - data.len() >= result2.start as usize);

  allocator.deallocate(result1);
  allocator.deallocate(result2);
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_code_allocation_protect_data_with_unwind_callbacks() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::{
      create_block_unwind_info_code_allocator_test::create_block_unwind_info_code_allocator_test,
      destroy_block_unwind_info_code_allocator_test::destroy_block_unwind_info_code_allocator_test,
    },
    records::info_code_allocator_test::Info,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  const K_CODE_ALIGNMENT: usize = 32;

  let _protect_data = ScopedFastFlag::new(&fflag::LuauCodegenProtectData, true);

  let mut info = Info {
    unwind: vec![0_u8; 8],
    ..Info::default()
  };

  {
    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    let code = [0_u8; 128];
    let data = [0_u8; 8];

    allocator.context = void_ptr(&mut info);
    allocator.create_block_unwind_info = Some(create_block_unwind_info_code_allocator_test);
    allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info_code_allocator_test);

    let result = allocate(&mut allocator, &data, &code);
    assert!(!result.start.is_null());
    assert_eq!(
      result.size,
      CodeAllocator::align_to_page_size(data.len()) + code.len()
    );
    assert!(!result.code_start.is_null());
    assert_eq!(
      result.code_start as usize,
      CodeAllocator::align_to_page_size(result.code_start as usize)
    );
    assert_eq!(info.block + K_CODE_ALIGNMENT, result.start as usize);

    allocator.deallocate(result);
  }

  assert!(info.destroy_called);
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_code_allocation_with_unwind_callbacks() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::{
      create_block_unwind_info_code_allocator_test::create_block_unwind_info_code_allocator_test,
      destroy_block_unwind_info_code_allocator_test::destroy_block_unwind_info_code_allocator_test,
    },
    records::info_code_allocator_test::Info,
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  const K_CODE_ALIGNMENT: usize = 32;

  let _protect_data = ScopedFastFlag::new(&fflag::LuauCodegenProtectData, false);

  let mut info = Info {
    unwind: vec![0_u8; 8],
    ..Info::default()
  };

  {
    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    let code = [0_u8; 128];
    let data = [0_u8; 8];

    allocator.context = void_ptr(&mut info);
    allocator.create_block_unwind_info = Some(create_block_unwind_info_code_allocator_test);
    allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info_code_allocator_test);

    let result = allocate(&mut allocator, &data, &code);
    assert!(!result.start.is_null());
    assert_eq!(result.size, K_CODE_ALIGNMENT + 128);
    assert!(!result.code_start.is_null());
    assert_eq!(
      result.start as usize + K_CODE_ALIGNMENT,
      result.code_start as usize
    );
    assert_eq!(info.block + K_CODE_ALIGNMENT, result.start as usize);

    allocator.deallocate(result);
  }

  assert!(info.destroy_called);
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_dwarf2_unwind_codes_a64() {
  use ulua_code_gen::records::{
    register_a_64::RegisterA64, unwind_builder::UnwindBuilder,
    unwind_builder_dwarf_2::UnwindBuilderDwarf2,
  };

  let mut unwind = UnwindBuilderDwarf2::default();

  unwind.start_info(UnwindBuilder::A64);
  unwind.start_function();
  unwind.prologue_a_64(
    28,
    64,
    &[
      RegisterA64::X29,
      RegisterA64::X30,
      RegisterA64::X19,
      RegisterA64::X20,
      RegisterA64::X21,
      RegisterA64::X22,
      RegisterA64::X23,
      RegisterA64::X24,
    ],
  );
  unwind.finish_function(0, 32);
  unwind.finish_info();

  let mut data = vec![0_u8; unwind.get_unwind_info_size(0)];
  finalize_dwarf2(&unwind, &mut data);

  let expected = vec![
    0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x78, 0x1e, 0x0c, 0x1f, 0x00,
    0x2c, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04, 0x0e, 0x40, 0x02, 0x18, 0x9d, 0x08,
    0x9e, 0x07, 0x93, 0x06, 0x94, 0x05, 0x95, 0x04, 0x96, 0x03, 0x97, 0x02, 0x98, 0x01, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
  ];

  assert_eq!(data.len(), expected.len());
  assert_eq!(data, expected);
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_dwarf2_unwind_codes_x64() {
  use ulua_code_gen::records::{
    register_x_64::RegisterX64, unwind_builder::UnwindBuilder,
    unwind_builder_dwarf_2::UnwindBuilderDwarf2,
  };

  let mut unwind = UnwindBuilderDwarf2::default();

  unwind.start_info(UnwindBuilder::X64);
  unwind.start_function();
  unwind.prologue_x_64(
    23,
    72,
    true,
    &[
      RegisterX64::RDI,
      RegisterX64::RSI,
      RegisterX64::RBX,
      RegisterX64::R12,
      RegisterX64::R13,
      RegisterX64::R14,
      RegisterX64::R15,
    ],
    &[],
  );
  unwind.finish_function(0, 0);
  unwind.finish_info();

  let mut data = vec![0_u8; unwind.get_unwind_info_size(0)];
  finalize_dwarf2(&unwind, &mut data);

  let expected = vec![
    0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x78, 0x10, 0x0c, 0x07, 0x08,
    0x90, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x02, 0x02, 0x0e, 0x10, 0x86, 0x02, 0x02, 0x03, 0x02, 0x02, 0x0e, 0x18, 0x85, 0x03, 0x02, 0x02,
    0x0e, 0x20, 0x84, 0x04, 0x02, 0x02, 0x0e, 0x28, 0x83, 0x05, 0x02, 0x02, 0x0e, 0x30, 0x8c, 0x06,
    0x02, 0x02, 0x0e, 0x38, 0x8d, 0x07, 0x02, 0x02, 0x0e, 0x40, 0x8e, 0x08, 0x02, 0x02, 0x0e, 0x48,
    0x8f, 0x09, 0x02, 0x04, 0x0e, 0x90, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
  ];

  assert_eq!(data.len(), expected.len());
  assert_eq!(data, expected);
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_generated_code_execution_a64() {
  #[cfg(not(target_arch = "aarch64"))]
  {
    return;
  }

  #[cfg(target_arch = "aarch64")]
  {
    use core::{ffi::c_int, mem};

    use ulua_code_gen::{
      records::{
        assembly_builder_a_64::AssemblyBuilderA64, code_allocator::CodeAllocator, label::Label,
        register_a_64::RegisterA64,
      },
      type_aliases::mem::mem,
    };

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);

    let mut skip = Label::default();
    build.cbz(RegisterA64::X1, &mut skip);
    build.ldrsw(RegisterA64::X1, mem(RegisterA64::X1, 0));
    build.cbnz(RegisterA64::X1, &mut skip);
    build.mov_register_a_64_i32(RegisterA64::X1, 0);
    build.set_label_label(&mut skip);

    let one = 1_u8;
    build.adr_register_a_64_void_usize(
      RegisterA64::X2,
      (&one as *const u8).cast(),
      mem::size_of_val(&one),
    );
    build.ldrb(RegisterA64::W2, mem(RegisterA64::X2, 0));
    build.sub_register_a_64_register_a_64_register_a_64_i32(
      RegisterA64::X1,
      RegisterA64::X1,
      RegisterA64::X2,
      0,
    );

    build.add_register_a_64_register_a_64_u16(RegisterA64::X1, RegisterA64::X1, 2);
    build.add_register_a_64_register_a_64_register_a_64_i32(
      RegisterA64::X0,
      RegisterA64::X0,
      RegisterA64::X1,
      1,
    );

    build.ret();
    assert!(build.finalize());

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    let code_allocation = allocate_u32_code(&mut allocator, &build.data, &build.code);
    assert!(!code_allocation.code_start.is_null());

    type FunctionType = extern "C-unwind" fn(i64, *mut c_int) -> i64;
    let f: FunctionType = entry(code_allocation.code_start, 0);

    let mut input = 10;
    let result = f(20, &mut input);
    assert_eq!(result, 42);

    allocator.deallocate(code_allocation);
  }
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_generated_code_execution_multiple_functions_with_throw_x64() {
  #[cfg(not(target_arch = "x86_64"))]
  {}

  #[cfg(target_arch = "x86_64")]
  {
    #[cfg(not(windows))]
    use ulua_code_gen::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;
    #[cfg(windows)]
    use ulua_code_gen::records::unwind_builder_win::UnwindBuilderWin;
    use ulua_code_gen::{
      functions::{
        create_block_unwind_info::create_block_unwind_info,
        destroy_block_unwind_info::destroy_block_unwind_info, is_supported::is_supported,
      },
      records::{
        assembly_builder_x_64::AssemblyBuilderX64, code_allocator::CodeAllocator, label::Label,
        register_x_64::RegisterX64 as R, unwind_builder::UnwindBuilder,
      },
    };
    use ulua_unit_test::functions::{
      assert_code_allocator_testing_panic::assert_code_allocator_testing_panic,
      throwing_code_allocator_test::throwing,
    };

    if !is_supported() {
      return;
    }

    #[cfg(windows)]
    let (r_arg1, r_arg2) = (R::RCX, R::RDX);
    #[cfg(not(windows))]
    let (r_arg1, r_arg2) = (R::RDI, R::RSI);

    const R_NON_VOL1: R = R::R12;
    const R_NON_VOL2: R = R::RBX;
    const R_NON_VOL3: R = R::R13;
    const R_NON_VOL4: R = R::R14;

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    #[cfg(windows)]
    let mut unwind = UnwindBuilderWin::default();
    #[cfg(not(windows))]
    let mut unwind = UnwindBuilderDwarf2::default();

    unwind.start_info(UnwindBuilder::X64);

    let mut start1 = Label::default();
    let mut start2 = Label::default();

    build.set_label_label(&mut start1);
    unwind.start_function();

    build.push(R::RBP.into());
    build.mov(R::RBP.into(), R::RSP.into());
    build.push(R_NON_VOL1.into());
    build.push(R_NON_VOL2.into());

    let stack_size = 32;
    let locals_size = 16;
    let frame_size = stack_size + locals_size;

    build.sub(R::RSP.into(), frame_size.into());

    let mut prologue_end1 = Label::default();
    build.set_label_label(&mut prologue_end1);
    let prologue_size = prologue_end1.location - start1.location;

    unwind.prologue_x_64(
      prologue_size,
      frame_size as u32,
      true,
      &[R_NON_VOL1, R_NON_VOL2],
      &[],
    );

    build.mov(R_NON_VOL1.into(), r_arg1.into());
    build.mov(R_NON_VOL2.into(), r_arg2.into());

    build.add(R_NON_VOL1.into(), 15.into());
    build.mov(r_arg1.into(), R_NON_VOL1.into());
    build.call_operand_x_64(R_NON_VOL2.into());

    build.add(R::RSP.into(), frame_size.into());
    build.pop(R_NON_VOL2.into());
    build.pop(R_NON_VOL1.into());
    build.pop(R::RBP.into());
    build.ret();

    let mut end1 = Label::default();
    build.set_label_label(&mut end1);
    unwind.finish_function(
      build.get_label_offset(&start1),
      build.get_label_offset(&end1),
    );

    build.set_label_label(&mut start2);
    unwind.start_function();

    build.push(R_NON_VOL1.into());
    build.push(R_NON_VOL2.into());
    build.push(R_NON_VOL3.into());
    build.push(R_NON_VOL4.into());

    let stack_size = 32;
    let locals_size = 24;
    let frame_size = stack_size + locals_size;

    build.sub(R::RSP.into(), frame_size.into());

    let mut prologue_end2 = Label::default();
    build.set_label_label(&mut prologue_end2);
    let prologue_size = prologue_end2.location - start2.location;

    unwind.prologue_x_64(
      prologue_size,
      frame_size as u32,
      false,
      &[R_NON_VOL1, R_NON_VOL2, R_NON_VOL3, R_NON_VOL4],
      &[],
    );

    build.mov(R_NON_VOL3.into(), r_arg1.into());
    build.mov(R_NON_VOL4.into(), r_arg2.into());

    build.add(R_NON_VOL3.into(), 15.into());
    build.mov(r_arg1.into(), R_NON_VOL3.into());
    build.call_operand_x_64(R_NON_VOL4.into());

    build.add(R::RSP.into(), frame_size.into());
    build.pop(R_NON_VOL4.into());
    build.pop(R_NON_VOL3.into());
    build.pop(R_NON_VOL2.into());
    build.pop(R_NON_VOL1.into());
    build.ret();

    unwind.finish_function(build.get_label_offset(&start2), u32::MAX);

    assert!(build.finalize());
    unwind.finish_info();

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    allocator.context = void_ptr(&mut unwind);
    allocator.create_block_unwind_info = Some(create_block_unwind_info);
    allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

    let code_allocation = allocate(&mut allocator, &build.data, &build.code);
    assert!(!code_allocation.code_start.is_null());

    type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
    let f1: FunctionType = entry(code_allocation.code_start, start1.location as usize);
    let f2: FunctionType = entry(code_allocation.code_start, start2.location as usize);

    let result = std::panic::catch_unwind(|| {
      let _ = f1(10, throwing);
    });
    assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

    let result = std::panic::catch_unwind(|| {
      let _ = f2(10, throwing);
    });
    assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

    allocator.deallocate(code_allocation);
  }
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_generated_code_execution_with_throw_a64() {
  #[cfg(not(target_arch = "aarch64"))]
  {
    return;
  }

  #[cfg(target_arch = "aarch64")]
  {
    use std::panic::catch_unwind;

    use ulua_code_gen::{
      functions::{
        create_block_unwind_info::create_block_unwind_info,
        destroy_block_unwind_info::destroy_block_unwind_info,
        is_unwind_supported::is_unwind_supported,
      },
      records::{
        assembly_builder_a_64::AssemblyBuilderA64, code_allocator::CodeAllocator,
        register_a_64::RegisterA64, unwind_builder::UnwindBuilder,
        unwind_builder_dwarf_2::UnwindBuilderDwarf2,
      },
      type_aliases::mem::mem,
    };
    use ulua_unit_test::functions::{
      assert_code_allocator_testing_panic::assert_code_allocator_testing_panic,
      throwing_code_allocator_test::throwing,
    };

    if !is_unwind_supported() {
      return;
    }

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    let mut unwind = UnwindBuilderDwarf2::default();

    unwind.start_info(UnwindBuilder::A64);

    build.sub_register_a_64_register_a_64_u16(RegisterA64::SP, RegisterA64::SP, 32);
    build.stp(RegisterA64::X29, RegisterA64::X30, mem(RegisterA64::SP, 0));
    build.str(RegisterA64::X28, mem(RegisterA64::SP, 16));
    build.mov_register_a_64_register_a_64(RegisterA64::X29, RegisterA64::SP);

    let prologue_end = build.set_label();

    build.add_register_a_64_register_a_64_u16(RegisterA64::X0, RegisterA64::X0, 15);
    build.blr(RegisterA64::X1);

    build.ldr(RegisterA64::X28, mem(RegisterA64::SP, 16));
    build.ldp(RegisterA64::X29, RegisterA64::X30, mem(RegisterA64::SP, 0));
    build.add_register_a_64_register_a_64_u16(RegisterA64::SP, RegisterA64::SP, 32);
    build.ret();

    let function_end = build.set_label();

    unwind.start_function();
    unwind.prologue_a_64(
      build.get_label_offset(&prologue_end),
      32,
      &[RegisterA64::X29, RegisterA64::X30, RegisterA64::X28],
    );
    unwind.finish_function(0, build.get_label_offset(&function_end));

    assert!(build.finalize());
    unwind.finish_info();

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    allocator.context = void_ptr(&mut unwind);
    allocator.create_block_unwind_info = Some(create_block_unwind_info);
    allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

    let code_allocation = allocate_u32_code(&mut allocator, &build.data, &build.code);
    assert!(!code_allocation.code_start.is_null());

    type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
    let f: FunctionType = entry(code_allocation.code_start, 0);

    let result = catch_unwind(|| {
      let _ = f(10, throwing);
    });

    assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

    allocator.deallocate(code_allocation);
  }
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_generated_code_execution_with_throw_outside_the_gate_x64() {
  #[cfg(not(target_arch = "x86_64"))]
  {}

  #[cfg(target_arch = "x86_64")]
  {
    #[cfg(not(windows))]
    use ulua_code_gen::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;
    #[cfg(windows)]
    use ulua_code_gen::records::unwind_builder_win::UnwindBuilderWin;
    use ulua_code_gen::{
      functions::{
        create_block_unwind_info::create_block_unwind_info,
        destroy_block_unwind_info::destroy_block_unwind_info, is_supported::is_supported,
      },
      records::{
        assembly_builder_x_64::AssemblyBuilderX64, code_allocator::CodeAllocator, label::Label,
        register_x_64::RegisterX64 as R, unwind_builder::UnwindBuilder,
      },
    };
    use ulua_unit_test::functions::{
      assert_code_allocator_testing_panic::assert_code_allocator_testing_panic,
      throwing_code_allocator_test::throwing,
    };

    if !is_supported() {
      return;
    }

    #[cfg(windows)]
    let (r_arg1, r_arg2, r_arg3) = (R::RCX, R::RDX, R::R8);
    #[cfg(not(windows))]
    let (r_arg1, r_arg2, r_arg3) = (R::RDI, R::RSI, R::RDX);

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    #[cfg(windows)]
    let mut unwind = UnwindBuilderWin::default();
    #[cfg(not(windows))]
    let mut unwind = UnwindBuilderDwarf2::default();

    unwind.start_info(UnwindBuilder::X64);

    let mut function_begin = Label::default();
    build.set_label(&mut function_begin);
    unwind.start_function();

    build.push(R::RBP.into());
    build.mov(R::RBP.into(), R::RSP.into());
    build.push(R::R10.into());
    build.push(R::R11.into());
    build.push(R::R12.into());
    build.push(R::R13.into());
    build.push(R::R14.into());
    build.push(R::R15.into());

    let stack_size = 64;
    let locals_size = 16;
    let frame_size = stack_size + locals_size;

    build.sub(R::RSP.into(), frame_size.into());

    let mut prologue_end = Label::default();
    build.set_label(&mut prologue_end);
    let prologue_size = prologue_end.location;

    unwind.prologue_x_64(
      prologue_size,
      frame_size as u32,
      true,
      &[R::R10, R::R11, R::R12, R::R13, R::R14, R::R15],
      &[],
    );

    build.mov(R::RAX.into(), r_arg1.into());
    build.mov(r_arg1.into(), 25.into());
    build.jmp_operand_x_64(R::RAX.into());

    let mut return_offset = Label::default();
    build.set_label(&mut return_offset);

    build.add(R::RSP.into(), frame_size.into());
    build.pop(R::R15.into());
    build.pop(R::R14.into());
    build.pop(R::R13.into());
    build.pop(R::R12.into());
    build.pop(R::R11.into());
    build.pop(R::R10.into());
    build.pop(R::RBP.into());
    build.ret();

    unwind.finish_function(build.get_label_offset(&function_begin), u32::MAX);

    assert!(build.finalize());
    unwind.finish_info();

    let block_size = 4096;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    allocator.context = void_ptr(&mut unwind);
    allocator.create_block_unwind_info = Some(create_block_unwind_info);
    allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

    let code_allocation1 = allocate(&mut allocator, &build.data, &build.code);
    assert!(!code_allocation1.code_start.is_null());

    unwind.set_begin_offset(prologue_size as usize);

    type FunctionType =
      extern "C-unwind" fn(*mut c_void, extern "C-unwind" fn(i64), *mut c_void) -> i64;
    let f: FunctionType = entry(code_allocation1.code_start, 0);

    // nativeExit：跳板函数体内部的返回入口（JIT 块内地址，按 cpp 同款以裸地址传参）。
    let native_exit = code_allocation1.code_start as usize + return_offset.location as usize;

    let mut build2 = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
    build2.mov(R::R12.into(), r_arg3.into());
    build2.call_operand_x_64(r_arg2.into());
    build2.jmp_operand_x_64(R::R12.into());
    assert!(build2.finalize());

    let code_allocation2 = allocate(&mut allocator, &build2.data, &build2.code);
    assert!(!code_allocation2.code_start.is_null());

    let result = std::panic::catch_unwind(|| {
      let _ = f(
        code_allocation2.code_start.cast(),
        throwing,
        native_exit as *mut c_void,
      );
    });
    assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

    allocator.deallocate(code_allocation1);
    allocator.deallocate(code_allocation2);
  }
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_generated_code_execution_with_throw_x64() {
  #[cfg(not(target_arch = "x86_64"))]
  {}

  #[cfg(target_arch = "x86_64")]
  {
    #[cfg(not(windows))]
    use ulua_code_gen::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;
    #[cfg(windows)]
    use ulua_code_gen::records::unwind_builder_win::UnwindBuilderWin;
    use ulua_code_gen::{
      functions::{
        create_block_unwind_info::create_block_unwind_info,
        destroy_block_unwind_info::destroy_block_unwind_info, is_supported::is_supported,
      },
      records::{
        assembly_builder_x_64::AssemblyBuilderX64, code_allocator::CodeAllocator, label::Label,
        register_x_64::RegisterX64 as R, unwind_builder::UnwindBuilder,
      },
    };
    use ulua_unit_test::functions::{
      assert_code_allocator_testing_panic::assert_code_allocator_testing_panic,
      nonthrowing::nonthrowing, throwing_code_allocator_test::throwing,
    };

    if !is_supported() {
      return;
    }

    #[cfg(windows)]
    let (r_arg1, r_arg2) = (R::RCX, R::RDX);
    #[cfg(not(windows))]
    let (r_arg1, r_arg2) = (R::RDI, R::RSI);

    const R_NON_VOL1: R = R::R12;
    const R_NON_VOL2: R = R::RBX;

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    #[cfg(windows)]
    let mut unwind = UnwindBuilderWin::default();
    #[cfg(not(windows))]
    let mut unwind = UnwindBuilderDwarf2::default();

    unwind.start_info(UnwindBuilder::X64);

    let mut function_begin = Label::default();
    build.set_label(&mut function_begin);
    unwind.start_function();

    build.push(R::RBP.into());
    build.mov(R::RBP.into(), R::RSP.into());
    build.push(R_NON_VOL1.into());
    build.push(R_NON_VOL2.into());

    let stack_size = 32;
    let locals_size = 16;

    build.sub(R::RSP.into(), (stack_size + locals_size).into());

    let mut prologue_end = Label::default();
    build.set_label(&mut prologue_end);
    let prologue_size = prologue_end.location;

    unwind.prologue_x_64(
      prologue_size,
      (stack_size + locals_size) as u32,
      true,
      &[R_NON_VOL1, R_NON_VOL2],
      &[],
    );

    build.mov(R_NON_VOL1.into(), r_arg1.into());
    build.mov(R_NON_VOL2.into(), r_arg2.into());

    build.add(R_NON_VOL1.into(), 15.into());
    build.mov(r_arg1.into(), R_NON_VOL1.into());
    build.call_operand_x_64(R_NON_VOL2.into());

    build.add(R::RSP.into(), (stack_size + locals_size).into());
    build.pop(R_NON_VOL2.into());
    build.pop(R_NON_VOL1.into());
    build.pop(R::RBP.into());
    build.ret();

    unwind.finish_function(build.get_label_offset(&function_begin), u32::MAX);

    assert!(build.finalize());
    unwind.finish_info();

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    allocator.context = void_ptr(&mut unwind);
    allocator.create_block_unwind_info = Some(create_block_unwind_info);
    allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

    let code_allocation = allocate(&mut allocator, &build.data, &build.code);
    assert!(!code_allocation.code_start.is_null());

    type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
    let f: FunctionType = entry(code_allocation.code_start, 0);

    let _ = f(10, nonthrowing);

    let result = std::panic::catch_unwind(|| {
      let _ = f(10, throwing);
    });

    assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

    allocator.deallocate(code_allocation);
  }
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_generated_code_execution_with_throw_x64_simd() {
  #[cfg(not(target_arch = "x86_64"))]
  {}

  #[cfg(target_arch = "x86_64")]
  {
    #[cfg(not(windows))]
    use ulua_code_gen::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;
    #[cfg(windows)]
    use ulua_code_gen::records::unwind_builder_win::UnwindBuilderWin;
    use ulua_code_gen::{
      enums::abix_64::ABIX64,
      functions::{
        create_block_unwind_info::create_block_unwind_info,
        destroy_block_unwind_info::destroy_block_unwind_info, is_supported::is_supported,
      },
      records::{
        assembly_builder_x_64::AssemblyBuilderX64, code_allocator::CodeAllocator, label::Label,
        operand_x_64::XMMWORD, register_x_64::RegisterX64 as R, unwind_builder::UnwindBuilder,
      },
    };
    use ulua_unit_test::functions::{
      nonthrowing::nonthrowing, obscure_throw_case::obscure_throw_case,
    };

    if !is_supported() {
      return;
    }

    #[cfg(windows)]
    let (r_arg1, r_arg2) = (R::RCX, R::RDX);
    #[cfg(not(windows))]
    let (r_arg1, r_arg2) = (R::RDI, R::RSI);

    const R_NON_VOL1: R = R::R12;
    const R_NON_VOL2: R = R::RBX;

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    #[cfg(windows)]
    let mut unwind = UnwindBuilderWin::default();
    #[cfg(not(windows))]
    let mut unwind = UnwindBuilderDwarf2::default();

    unwind.start_info(UnwindBuilder::X64);

    let mut function_begin = Label::default();
    build.set_label(&mut function_begin);
    unwind.start_function();

    let stack_size = 32 + 64;
    let locals_size = 16;
    let frame_size = stack_size + locals_size;

    build.push(R_NON_VOL1.into());
    build.push(R_NON_VOL2.into());
    build.push(R::RBP.into());
    build.sub(R::RSP.into(), frame_size.into());

    if build.abi == ABIX64::Windows {
      build.vmovaps(
        XMMWORD.operator_bracket(R::RSP + (frame_size - 0x40)),
        R::XMM6.into(),
      );
      build.vmovaps(
        XMMWORD.operator_bracket(R::RSP + (frame_size - 0x30)),
        R::XMM7.into(),
      );
      build.vmovaps(
        XMMWORD.operator_bracket(R::RSP + (frame_size - 0x20)),
        R::XMM8.into(),
      );
      build.vmovaps(
        XMMWORD.operator_bracket(R::RSP + (frame_size - 0x10)),
        R::XMM9.into(),
      );
    }

    let mut prologue_end = Label::default();
    build.set_label(&mut prologue_end);
    let prologue_size = prologue_end.location;

    if build.abi == ABIX64::Windows {
      unwind.prologue_x_64(
        prologue_size,
        frame_size as u32,
        false,
        &[R_NON_VOL1, R_NON_VOL2, R::RBP],
        &[R::XMM6, R::XMM7, R::XMM8, R::XMM9],
      );
    } else {
      unwind.prologue_x_64(
        prologue_size,
        frame_size as u32,
        false,
        &[R_NON_VOL1, R_NON_VOL2, R::RBP],
        &[],
      );
    }

    build.vxorpd(R::XMM0.into(), R::XMM0.into(), R::XMM0.into());
    build.vmovsd_operand_x_64_operand_x_64_operand_x_64(
      R::XMM6.into(),
      R::XMM0.into(),
      R::XMM0.into(),
    );
    build.vmovsd_operand_x_64_operand_x_64_operand_x_64(
      R::XMM7.into(),
      R::XMM0.into(),
      R::XMM0.into(),
    );
    build.vmovsd_operand_x_64_operand_x_64_operand_x_64(
      R::XMM8.into(),
      R::XMM0.into(),
      R::XMM0.into(),
    );
    build.vmovsd_operand_x_64_operand_x_64_operand_x_64(
      R::XMM9.into(),
      R::XMM0.into(),
      R::XMM0.into(),
    );

    build.mov(R_NON_VOL1.into(), r_arg1.into());
    build.mov(R_NON_VOL2.into(), r_arg2.into());

    build.add(R_NON_VOL1.into(), 15.into());
    build.mov(r_arg1.into(), R_NON_VOL1.into());
    build.call_operand_x_64(R_NON_VOL2.into());

    if build.abi == ABIX64::Windows {
      build.vmovaps(
        R::XMM6.into(),
        XMMWORD.operator_bracket(R::RSP + (frame_size - 0x40)),
      );
      build.vmovaps(
        R::XMM7.into(),
        XMMWORD.operator_bracket(R::RSP + (frame_size - 0x30)),
      );
      build.vmovaps(
        R::XMM8.into(),
        XMMWORD.operator_bracket(R::RSP + (frame_size - 0x20)),
      );
      build.vmovaps(
        R::XMM9.into(),
        XMMWORD.operator_bracket(R::RSP + (frame_size - 0x10)),
      );
    }

    build.add(R::RSP.into(), frame_size.into());
    build.pop(R::RBP.into());
    build.pop(R_NON_VOL2.into());
    build.pop(R_NON_VOL1.into());
    build.ret();

    unwind.finish_function(build.get_label_offset(&function_begin), u32::MAX);

    assert!(build.finalize());
    unwind.finish_info();

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    allocator.context = void_ptr(&mut unwind);
    allocator.create_block_unwind_info = Some(create_block_unwind_info);
    allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

    let code_allocation = allocate(&mut allocator, &build.data, &build.code);
    assert!(!code_allocation.code_start.is_null());

    type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
    let f: FunctionType = entry(code_allocation.code_start, 0);

    let _ = f(10, nonthrowing);
    obscure_throw_case(f);

    allocator.deallocate(code_allocation);
  }
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_generated_code_execution_x64() {
  #[cfg(not(target_arch = "x86_64"))]
  {}

  #[cfg(target_arch = "x86_64")]
  {
    use ulua_code_gen::{
      functions::is_supported::is_supported,
      records::{
        assembly_builder_x_64::AssemblyBuilderX64, code_allocator::CodeAllocator,
        register_x_64::RegisterX64 as R,
      },
    };

    if !is_supported() {
      return;
    }

    #[cfg(windows)]
    let (r_arg1, r_arg2) = (R::RCX, R::RDX);
    #[cfg(not(windows))]
    let (r_arg1, r_arg2) = (R::RDI, R::RSI);

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    build.mov(R::RAX.into(), r_arg1.into());
    build.add(R::RAX.into(), r_arg2.into());
    build.imul_operand_x_64_operand_x_64_i32(R::RAX.into(), R::RAX.into(), 7);
    build.ret();
    assert!(build.finalize());

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    let code_allocation = allocate(&mut allocator, &build.data, &build.code);
    assert!(!code_allocation.code_start.is_null());

    type FunctionType = extern "C-unwind" fn(i64, i64) -> i64;
    let f: FunctionType = entry(code_allocation.code_start, 0);
    let result = f(10, 20);
    assert_eq!(result, 210);

    allocator.deallocate(code_allocation);
  }
}

// Source: `tests/CodeAllocator.test.cpp`
#[test]
fn code_allocator_windows_unwind_codes_x64() {
  use ulua_code_gen::records::{
    register_x_64::RegisterX64, unwind_builder::UnwindBuilder, unwind_builder_win::UnwindBuilderWin,
  };

  let mut unwind = UnwindBuilderWin::default();

  unwind.start_info(UnwindBuilder::X64);
  unwind.start_function();
  unwind.prologue_x_64(
    23,
    72,
    true,
    &[
      RegisterX64::RDI,
      RegisterX64::RSI,
      RegisterX64::RBX,
      RegisterX64::R12,
      RegisterX64::R13,
      RegisterX64::R14,
      RegisterX64::R15,
    ],
    &[],
  );
  unwind.finish_function(0x11223344, 0x55443322);
  unwind.finish_info();

  let mut data = vec![0_u8; unwind.get_unwind_info_size(0)];
  unwind.finalize(data.as_mut_ptr().cast(), 0, null_mut(), 0);

  let expected = vec![
    0x44, 0x33, 0x22, 0x11, 0x22, 0x33, 0x44, 0x55, 0x0c, 0x00, 0x00, 0x00, 0x01, 0x17, 0x0a, 0x05,
    0x17, 0x82, 0x13, 0xf0, 0x11, 0xe0, 0x0f, 0xd0, 0x0d, 0xc0, 0x0b, 0x30, 0x09, 0x60, 0x07, 0x70,
    0x05, 0x03, 0x02, 0x50,
  ];

  assert_eq!(data.len(), expected.len());
  assert_eq!(data, expected);
}
