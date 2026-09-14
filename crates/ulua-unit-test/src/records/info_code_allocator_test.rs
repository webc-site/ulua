//! Generated skeleton item.
//! Node: `cxx:Record:Luau.UnitTest:tests/CodeAllocator.test.cpp:141:info`
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
//!   - type_ref <- record Info (tests/CodeAllocator.test.cpp)
//! - outgoing:
//!   - type_ref -> record Info (tests/CodeAllocator.test.cpp)
//!   - translates_to -> rust_item Info

use alloc::vec::Vec;
use core::ptr::null_mut;
#[derive(Debug, Clone)]
pub struct Info {
  pub unwind: Vec<u8>,
  pub block: *mut u8,
  pub destroy_called: bool,
}

impl Default for Info {
  fn default() -> Self {
    Self {
      unwind: Vec::new(),
      block: null_mut(),
      destroy_called: false,
    }
  }
}
