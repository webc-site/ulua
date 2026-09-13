//! Node: `cxx:Method:Luau.CodeGen:CodeGen/include/Luau/IrRegAllocX64.h:127:scoped_reg_x_64_scoped_reg_x_64`
//! Source: `CodeGen/include/Luau/IrRegAllocX64.h`
//!
//! C++ `ScopedRegX64(const ScopedRegX64&) = delete;` — the deleted copy constructor.
//! `ScopedRegX64` owns a unique borrow of its register allocator (`owner`) and is
//! move-only; the copy constructor is deleted in C++ and must never be invoked.
//! This faithfully encodes the deletion (panics if ever called), matching the
//! project convention for `= delete` members (cf. `RecursionCounter`).
use crate::records::scoped_reg_x_64::ScopedRegX64;

impl ScopedRegX64 {
  pub fn scoped_reg_x_64_scoped_reg_x_64_copy(&mut self) {
    unimplemented!("ScopedRegX64 copy constructor is deleted in C++");
  }
}
