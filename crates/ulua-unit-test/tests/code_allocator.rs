use core::{
  ffi::c_int,
  ptr::{null, null_mut},
};
use std::panic::catch_unwind;
extern crate alloc;

mod code_allocator_code_allocation {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:26:code_allocator_code_allocation`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_code_allocation
  use super::*;

  #[cfg(test)]
  #[test]
  fn code_allocator_code_allocation() {
    use ulua_code_gen::records::code_allocator::CodeAllocator;
    use ulua_common::FFlag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    const K_CODE_ALIGNMENT: usize = 32;

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);
    let _protect_data = ScopedFastFlag::new(&FFlag::LuauCodegenProtectData, false);

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    let code = [0_u8; 128];

    let result1 = unsafe { allocator.allocate(null(), 0, code.as_ptr(), code.len()) };
    assert!(!result1.start.is_null());
    assert_eq!(result1.size, 128);
    assert!(!result1.code_start.is_null());
    assert_eq!(result1.code_start, result1.start);

    let data = [0_u8; 8];

    let result2 =
      unsafe { allocator.allocate(data.as_ptr(), data.len(), code.as_ptr(), code.len()) };
    assert!(!result2.start.is_null());
    assert_eq!(result2.size, K_CODE_ALIGNMENT + 128);
    assert!(!result2.code_start.is_null());
    assert_eq!(
      unsafe { result2.start.add(K_CODE_ALIGNMENT) },
      result2.code_start
    );

    allocator.deallocate(result1);
    allocator.deallocate(result2);
  }
}

mod code_allocator_code_allocation_callbacks {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:57:code_allocator_code_allocation_callbacks`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record AllocationData (tests/CodeAllocator.test.cpp)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_code_allocation_callbacks
  use super::*;

  #[cfg(test)]
  #[test]
  fn code_allocator_code_allocation_callbacks() {
    use ulua_code_gen::records::code_allocator::CodeAllocator;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::allocation_callback_code_allocator_test::allocation_callback_code_allocator_test,
      records::allocation_data::AllocationData, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocation_data = AllocationData::default();

    {
      let mut allocator = CodeAllocator::default();
      allocator.code_allocator_usize_usize_allocation_callback_void(
        block_size,
        max_total_size,
        Some(allocation_callback_code_allocator_test),
        (&mut allocation_data as *mut AllocationData).cast(),
      );

      let code = [0_u8; 128];

      let result = unsafe { allocator.allocate(null(), 0, code.as_ptr(), code.len()) };
      assert!(!result.start.is_null());
      assert_eq!(allocation_data.bytes_allocated, block_size);
      assert_eq!(allocation_data.bytes_freed, 0);

      allocator.deallocate(result);
    }

    assert_eq!(allocation_data.bytes_allocated, block_size);
    assert_eq!(allocation_data.bytes_freed, block_size);
  }
}

mod code_allocator_code_allocation_failure {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:108:code_allocator_code_allocation_failure`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_code_allocation_failure
  use super::*;

  #[cfg(test)]
  #[test]
  fn code_allocator_code_allocation_failure() {
    use ulua_code_gen::records::code_allocator::CodeAllocator;
    use ulua_common::FFlag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

    let block_size = 3000;
    let max_total_size = 7000;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    let mut code = vec![0_u8; 4000];

    let result1 = unsafe { allocator.allocate(null(), 0, code.as_ptr(), code.len()) };
    assert!(result1.start.is_null());

    code.resize(2000, 0);
    let result2 = unsafe { allocator.allocate(null(), 0, code.as_ptr(), code.len()) };
    assert!(!result2.start.is_null());
    let result3 = unsafe { allocator.allocate(null(), 0, code.as_ptr(), code.len()) };
    assert!(!result3.start.is_null());
    let result4 = unsafe { allocator.allocate(null(), 0, code.as_ptr(), code.len()) };
    assert!(result4.start.is_null());

    allocator.deallocate(result2);
    allocator.deallocate(result3);
  }
}

mod code_allocator_code_allocation_protect_data {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:197:code_allocator_code_allocation_protect_data`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method AssemblyBuilderX64::align (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method CodeAllocator::alignToPageSize (CodeGen/src/CodeAllocator.cpp)
  //!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_code_allocation_protect_data
  use super::*;

  #[cfg(test)]
  #[test]
  fn code_allocator_code_allocation_protect_data() {
    use ulua_code_gen::records::code_allocator::CodeAllocator;
    use ulua_common::FFlag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);
    let _protect_data = ScopedFastFlag::new(&FFlag::LuauCodegenProtectData, true);

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    let code = [0_u8; 128];
    let result1 = unsafe { allocator.allocate(null(), 0, code.as_ptr(), code.len()) };
    assert!(!result1.start.is_null());
    assert_eq!(result1.size, 128);
    assert!(!result1.code_start.is_null());
    assert_eq!(result1.code_start, result1.start);

    let data = [0_u8; 8];
    let result2 =
      unsafe { allocator.allocate(data.as_ptr(), data.len(), code.as_ptr(), code.len()) };
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
    assert!(unsafe { result2.code_start.sub(data.len()) } >= result2.start);

    allocator.deallocate(result1);
    allocator.deallocate(result2);
  }
}

mod code_allocator_code_allocation_protect_data_with_unwind_callbacks {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:229:code_allocator_code_allocation_protect_data_with_unwind_callbacks`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method CodeAllocator::alignToPageSize (CodeGen/src/CodeAllocator.cpp)
  //!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_code_allocation_protect_data_with_unwind_callbacks

  #[cfg(test)]
  #[test]
  fn code_allocator_code_allocation_protect_data_with_unwind_callbacks() {
    use ulua_code_gen::records::code_allocator::CodeAllocator;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::{
        create_block_unwind_info_code_allocator_test_alt_b::create_block_unwind_info_code_allocator_test_alt_b,
        destroy_block_unwind_info_code_allocator_test_alt_b::destroy_block_unwind_info_code_allocator_test_alt_b,
      },
      records::info_code_allocator_test_alt_b::Info,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const K_CODE_ALIGNMENT: usize = 32;

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);
    let _protect_data = ScopedFastFlag::new(&FFlag::LuauCodegenProtectData, true);

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

      allocator.context = (&mut info as *mut Info).cast();
      allocator.create_block_unwind_info = Some(create_block_unwind_info_code_allocator_test_alt_b);
      allocator.destroy_block_unwind_info =
        Some(destroy_block_unwind_info_code_allocator_test_alt_b);

      let result =
        unsafe { allocator.allocate(data.as_ptr(), data.len(), code.as_ptr(), code.len()) };
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
      assert_eq!(unsafe { info.block.add(K_CODE_ALIGNMENT) }, result.start);

      allocator.deallocate(result);
    }

    assert!(info.destroy_called);
  }
}

mod code_allocator_code_allocation_with_unwind_callbacks {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:136:code_allocator_code_allocation_with_unwind_callbacks`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_code_allocation_with_unwind_callbacks

  #[cfg(test)]
  #[test]
  fn code_allocator_code_allocation_with_unwind_callbacks() {
    use ulua_code_gen::records::code_allocator::CodeAllocator;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::{
        create_block_unwind_info_code_allocator_test::create_block_unwind_info_code_allocator_test,
        destroy_block_unwind_info_code_allocator_test::destroy_block_unwind_info_code_allocator_test,
      },
      records::info_code_allocator_test::Info,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    const K_CODE_ALIGNMENT: usize = 32;

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);
    let _protect_data = ScopedFastFlag::new(&FFlag::LuauCodegenProtectData, false);

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

      allocator.context = (&mut info as *mut Info).cast();
      allocator.create_block_unwind_info = Some(create_block_unwind_info_code_allocator_test);
      allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info_code_allocator_test);

      let result =
        unsafe { allocator.allocate(data.as_ptr(), data.len(), code.as_ptr(), code.len()) };
      assert!(!result.start.is_null());
      assert_eq!(result.size, K_CODE_ALIGNMENT + 128);
      assert!(!result.code_start.is_null());
      assert_eq!(
        unsafe { result.start.add(K_CODE_ALIGNMENT) },
        result.code_start
      );
      assert_eq!(unsafe { info.block.add(K_CODE_ALIGNMENT) }, result.start);

      allocator.deallocate(result);
    }

    assert!(info.destroy_called);
  }
}

mod code_allocator_dwarf2_unwind_codes_a64 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:343:code_allocator_dwarf2_unwind_codes_a64`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
  //!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
  //!   - translates_to -> rust_item code_allocator_dwarf2_unwind_codes_a64
  use super::*;

  #[cfg(test)]
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
    unsafe { unwind.finalize(data.as_mut_ptr().cast(), 0, null_mut(), 0) };

    let expected = vec![
      0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x78, 0x1e, 0x0c, 0x1f,
      0x00, 0x2c, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04, 0x0e, 0x40, 0x02,
      0x18, 0x9d, 0x08, 0x9e, 0x07, 0x93, 0x06, 0x94, 0x05, 0x95, 0x04, 0x96, 0x03, 0x97, 0x02,
      0x98, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    assert_eq!(data.len(), expected.len());
    assert_eq!(data, expected);
  }
}

mod code_allocator_dwarf2_unwind_codes_x64 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:316:code_allocator_dwarf2_unwind_codes_x64`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
  //!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
  //!   - translates_to -> rust_item code_allocator_dwarf2_unwind_codes_x64
  use super::*;

  #[cfg(test)]
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
    unsafe { unwind.finalize(data.as_mut_ptr().cast(), 0, null_mut(), 0) };

    let expected = vec![
      0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x78, 0x10, 0x0c, 0x07,
      0x08, 0x90, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x00, 0x00, 0x1c, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x02, 0x02, 0x0e, 0x10, 0x86, 0x02, 0x02, 0x03, 0x02, 0x02, 0x0e, 0x18,
      0x85, 0x03, 0x02, 0x02, 0x0e, 0x20, 0x84, 0x04, 0x02, 0x02, 0x0e, 0x28, 0x83, 0x05, 0x02,
      0x02, 0x0e, 0x30, 0x8c, 0x06, 0x02, 0x02, 0x0e, 0x38, 0x8d, 0x07, 0x02, 0x02, 0x0e, 0x40,
      0x8e, 0x08, 0x02, 0x02, 0x0e, 0x48, 0x8f, 0x09, 0x02, 0x04, 0x0e, 0x90, 0x01, 0x00, 0x00,
      0x00, 0x00, 0x00,
    ];

    assert_eq!(data.len(), expected.len());
    assert_eq!(data, expected);
  }
}

mod code_allocator_generated_code_execution_a64 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:890:code_allocator_generated_code_execution_a64`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record AssemblyBuilderA64 (CodeGen/include/Luau/AssemblyBuilderA64.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
  //!   - calls -> method AssemblyBuilderA64::cbz (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - calls -> method AssemblyBuilderA64::ldrsw (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - calls -> method AssemblyBuilderA64::cbnz (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - calls -> method AssemblyBuilderA64::ldrb (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_generated_code_execution_a64
  use core::mem;

  use super::*;

  #[cfg(test)]
  #[test]
  fn code_allocator_generated_code_execution_a64() {
    #[cfg(not(target_arch = "aarch64"))]
    {
      return;
    }

    #[cfg(target_arch = "aarch64")]
    {
      use ulua_code_gen::{
        records::{
          assembly_builder_a_64::AssemblyBuilderA64, code_allocator::CodeAllocator, label::Label,
          register_a_64::RegisterA64,
        },
        type_aliases::mem::mem,
      };
      use ulua_common::FFlag;
      use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

      let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

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

      let code_allocation = unsafe {
        allocator.allocate(
          build.data.as_ptr(),
          build.data.len(),
          build.code.as_ptr().cast(),
          build.code.len() * mem::size_of::<u32>(),
        )
      };
      assert!(!code_allocation.code_start.is_null());

      type FunctionType = extern "C-unwind" fn(i64, *mut c_int) -> i64;
      let f: FunctionType = unsafe { mem::transmute(code_allocation.code_start) };

      let mut input = 10;
      let result = f(20, &mut input);
      assert_eq!(result, 42);

      allocator.deallocate(code_allocation);
    }
  }
}

mod code_allocator_generated_code_execution_multiple_functions_with_throw_x64 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:635:code_allocator_generated_code_execution_multiple_functions_with_throw_x64`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
  //!   - type_ref -> record AssemblyBuilderX64 (CodeGen/include/Luau/AssemblyBuilderX64.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
  //!   - type_ref -> record UnwindBuilderWin (CodeGen/include/Luau/UnwindBuilderWin.h)
  //!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
  //!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
  //!   - type_ref -> record First (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_generated_code_execution_multiple_functions_with_throw_x64

  #[cfg(test)]
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
      use ulua_common::FFlag;
      use ulua_unit_test::{
        functions::{
          assert_code_allocator_testing_panic::assert_code_allocator_testing_panic,
          throwing_code_allocator_test_alt_b::throwing,
        },
        type_aliases::scoped_fast_flag::ScopedFastFlag,
      };

      let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

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

      allocator.context = &mut unwind as *mut _ as *mut core::ffi::c_void;
      allocator.create_block_unwind_info = Some(create_block_unwind_info);
      allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

      let code_allocation = unsafe {
        allocator.allocate(
          build.data.as_ptr(),
          build.data.len(),
          build.code.as_ptr(),
          build.code.len(),
        )
      };
      assert!(!code_allocation.code_start.is_null());

      type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
      let f1: FunctionType =
        unsafe { core::mem::transmute(code_allocation.code_start.add(start1.location as usize)) };
      let f2: FunctionType =
        unsafe { core::mem::transmute(code_allocation.code_start.add(start2.location as usize)) };

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
}

mod code_allocator_generated_code_execution_with_throw_a64 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:941:code_allocator_generated_code_execution_with_throw_a64`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function isUnwindSupported (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - type_ref -> record AssemblyBuilderA64 (CodeGen/include/Luau/AssemblyBuilderA64.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
  //!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
  //!   - calls -> method AssemblyBuilderA64::stp (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - calls -> type_alias mem (CodeGen/include/Luau/AddressA64.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
  //!   - calls -> method AssemblyBuilderA64::blr (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - calls -> method AssemblyBuilderA64::ldr (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - calls -> method AssemblyBuilderA64::ldp (CodeGen/src/AssemblyBuilderA64.cpp)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_generated_code_execution_with_throw_a64
  use core::mem;

  use super::*;

  #[cfg(test)]
  #[test]
  fn code_allocator_generated_code_execution_with_throw_a64() {
    #[cfg(not(target_arch = "aarch64"))]
    {
      return;
    }

    #[cfg(target_arch = "aarch64")]
    {
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
      use ulua_common::FFlag;
      use ulua_unit_test::{
        functions::{
          assert_code_allocator_testing_panic::assert_code_allocator_testing_panic,
          throwing_code_allocator_test_alt_b::throwing,
        },
        type_aliases::scoped_fast_flag::ScopedFastFlag,
      };

      let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

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

      allocator.context = (&mut unwind as *mut UnwindBuilderDwarf2).cast();
      allocator.create_block_unwind_info = Some(create_block_unwind_info);
      allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

      let code_allocation = unsafe {
        allocator.allocate(
          build.data.as_ptr(),
          build.data.len(),
          build.code.as_ptr().cast(),
          build.code.len() * mem::size_of::<u32>(),
        )
      };
      assert!(!code_allocation.code_start.is_null());

      type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
      let f: FunctionType = unsafe { mem::transmute(code_allocation.code_start) };

      let result = catch_unwind(|| {
        let _ = f(10, throwing);
      });

      assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

      allocator.deallocate(code_allocation);
    }
  }
}

mod code_allocator_generated_code_execution_with_throw_outside_the_gate_x64 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:776:code_allocator_generated_code_execution_with_throw_outside_the_gate_x64`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
  //!   - type_ref -> record AssemblyBuilderX64 (CodeGen/include/Luau/AssemblyBuilderX64.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
  //!   - type_ref -> record UnwindBuilderWin (CodeGen/include/Luau/UnwindBuilderWin.h)
  //!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
  //!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_generated_code_execution_with_throw_outside_the_gate_x64

  #[cfg(test)]
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
      use ulua_common::FFlag;
      use ulua_unit_test::{
        functions::{
          assert_code_allocator_testing_panic::assert_code_allocator_testing_panic,
          throwing_code_allocator_test_alt_b::throwing,
        },
        type_aliases::scoped_fast_flag::ScopedFastFlag,
      };

      let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

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

      allocator.context = &mut unwind as *mut _ as *mut core::ffi::c_void;
      allocator.create_block_unwind_info = Some(create_block_unwind_info);
      allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

      let code_allocation1 = unsafe {
        allocator.allocate(
          build.data.as_ptr(),
          build.data.len(),
          build.code.as_ptr(),
          build.code.len(),
        )
      };
      assert!(!code_allocation1.code_start.is_null());

      unwind.set_begin_offset(prologue_size as usize);

      type FunctionType = extern "C-unwind" fn(
        *mut core::ffi::c_void,
        extern "C-unwind" fn(i64),
        *mut core::ffi::c_void,
      ) -> i64;
      let f: FunctionType = unsafe { core::mem::transmute(code_allocation1.code_start) };

      let native_exit = unsafe {
        code_allocation1
          .code_start
          .add(return_offset.location as usize)
      };

      let mut build2 = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      build2.mov(R::R12.into(), r_arg3.into());
      build2.call_operand_x_64(r_arg2.into());
      build2.jmp_operand_x_64(R::R12.into());
      assert!(build2.finalize());

      let code_allocation2 = unsafe {
        allocator.allocate(
          build2.data.as_ptr(),
          build2.data.len(),
          build2.code.as_ptr(),
          build2.code.len(),
        )
      };
      assert!(!code_allocation2.code_start.is_null());

      let result = std::panic::catch_unwind(|| {
        let _ = f(
          code_allocation2.code_start.cast(),
          throwing,
          native_exit.cast(),
        );
      });
      assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

      allocator.deallocate(code_allocation1);
      allocator.deallocate(code_allocation2);
    }
  }
}

mod code_allocator_generated_code_execution_with_throw_x64 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:432:code_allocator_generated_code_execution_with_throw_x64`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
  //!   - type_ref -> record AssemblyBuilderX64 (CodeGen/include/Luau/AssemblyBuilderX64.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
  //!   - type_ref -> record UnwindBuilderWin (CodeGen/include/Luau/UnwindBuilderWin.h)
  //!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
  //!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> function nonthrowing (tests/CodeAllocator.test.cpp)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_generated_code_execution_with_throw_x64

  #[cfg(test)]
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
      use ulua_common::FFlag;
      use ulua_unit_test::{
        functions::{
          assert_code_allocator_testing_panic::assert_code_allocator_testing_panic,
          nonthrowing::nonthrowing, throwing_code_allocator_test_alt_b::throwing,
        },
        type_aliases::scoped_fast_flag::ScopedFastFlag,
      };

      let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

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

      allocator.context = &mut unwind as *mut _ as *mut core::ffi::c_void;
      allocator.create_block_unwind_info = Some(create_block_unwind_info);
      allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

      let code_allocation = unsafe {
        allocator.allocate(
          build.data.as_ptr(),
          build.data.len(),
          build.code.as_ptr(),
          build.code.len(),
        )
      };
      assert!(!code_allocation.code_start.is_null());

      type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
      let f: FunctionType = unsafe { core::mem::transmute(code_allocation.code_start) };

      let _ = f(10, nonthrowing);

      let result = std::panic::catch_unwind(|| {
        let _ = f(10, throwing);
      });

      assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

      allocator.deallocate(code_allocation);
    }
  }
}

mod code_allocator_generated_code_execution_with_throw_x64_simd {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:532:code_allocator_generated_code_execution_with_throw_x64_simd`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
  //!   - type_ref -> record AssemblyBuilderX64 (CodeGen/include/Luau/AssemblyBuilderX64.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
  //!   - type_ref -> record UnwindBuilderWin (CodeGen/include/Luau/UnwindBuilderWin.h)
  //!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
  //!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
  //!   - calls -> method AssemblyBuilderX64::vmovaps (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method AssemblyBuilderX64::vxorpd (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> function nonthrowing (tests/CodeAllocator.test.cpp)
  //!   - calls -> function obscureThrowCase (tests/CodeAllocator.test.cpp)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_generated_code_execution_with_throw_x64_simd

  #[cfg(test)]
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
      use ulua_common::FFlag;
      use ulua_unit_test::{
        functions::{nonthrowing::nonthrowing, obscure_throw_case::obscure_throw_case},
        type_aliases::scoped_fast_flag::ScopedFastFlag,
      };

      let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

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

      allocator.context = &mut unwind as *mut _ as *mut core::ffi::c_void;
      allocator.create_block_unwind_info = Some(create_block_unwind_info);
      allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

      let code_allocation = unsafe {
        allocator.allocate(
          build.data.as_ptr(),
          build.data.len(),
          build.code.as_ptr(),
          build.code.len(),
        )
      };
      assert!(!code_allocation.code_start.is_null());

      type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
      let f: FunctionType = unsafe { core::mem::transmute(code_allocation.code_start) };

      let _ = f(10, nonthrowing);
      obscure_throw_case(f);

      allocator.deallocate(code_allocation);
    }
  }
}

mod code_allocator_generated_code_execution_x64 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:387:code_allocator_generated_code_execution_x64`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
  //!   - type_ref -> record AssemblyBuilderX64 (CodeGen/include/Luau/AssemblyBuilderX64.h)
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
  //!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
  //!   - translates_to -> rust_item code_allocator_generated_code_execution_x64

  #[cfg(test)]
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
      use ulua_common::FFlag;
      use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

      let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

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

      let code_allocation = unsafe {
        allocator.allocate(
          build.data.as_ptr(),
          build.data.len(),
          build.code.as_ptr(),
          build.code.len(),
        )
      };
      assert!(!code_allocation.code_start.is_null());

      type FunctionType = extern "C-unwind" fn(i64, i64) -> i64;
      let f: FunctionType = unsafe { core::mem::transmute(code_allocation.code_start) };
      let result = f(10, 20);
      assert_eq!(result, 210);

      allocator.deallocate(code_allocation);
    }
  }
}

mod code_allocator_windows_unwind_codes_x64 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:292:code_allocator_windows_unwind_codes_x64`
  //! Source: `tests/CodeAllocator.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/CodeAllocator.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
  //!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
  //!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
  //!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file VM/src/lstring.h
  //! - incoming:
  //!   - declares <- source_file tests/CodeAllocator.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnwindBuilderWin (CodeGen/include/Luau/UnwindBuilderWin.h)
  //!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
  //!   - translates_to -> rust_item code_allocator_windows_unwind_codes_x64
  use super::*;

  #[cfg(test)]
  #[test]
  fn code_allocator_windows_unwind_codes_x64() {
    use ulua_code_gen::records::{
      register_x_64::RegisterX64, unwind_builder::UnwindBuilder,
      unwind_builder_win::UnwindBuilderWin,
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
      0x44, 0x33, 0x22, 0x11, 0x22, 0x33, 0x44, 0x55, 0x0c, 0x00, 0x00, 0x00, 0x01, 0x17, 0x0a,
      0x05, 0x17, 0x82, 0x13, 0xf0, 0x11, 0xe0, 0x0f, 0xd0, 0x0d, 0xc0, 0x0b, 0x30, 0x09, 0x60,
      0x07, 0x70, 0x05, 0x03, 0x02, 0x50,
    ];

    assert_eq!(data.len(), expected.len());
    assert_eq!(data, expected);
  }
}
