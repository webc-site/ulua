use crate::{
  enums::{abix_64::ABIX64, category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  methods::ir_call_wrapper_x_64_get_next_argument_target::{
    systemv_gpr_order, windows_gpr_order, xmm_order,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64,
    register_x_64::RegisterX64,
  },
};

impl IrCallWrapperX64 {
  // C++: `static RegisterX64 suggestArgumentRegister(SizeX64 size, AssemblyBuilderX64& build);`
  // (static template; instantiated for N = 0..3). No instance state is touched, so this is an
  // associated function in Rust as well -- callers invoke it as
  // `IrCallWrapperX64::suggest_argument_register::<N>(size, build)`.
  pub fn suggest_argument_register<const N: usize>(
    size: SizeX64,
    build: &mut AssemblyBuilderX64,
  ) -> RegisterX64 {
    // static_assert(N <= 3, "Argument index must be 0-3 (Windows passes args 4+ on the stack)");
    const { assert!(N <= 3) };

    if size == SizeX64::Xmmword {
      return xmm_order()[N].base;
    }

    let gpr_order = if build.abi == ABIX64::WINDOWS {
      windows_gpr_order()
    } else {
      systemv_gpr_order()
    };

    let mut target = gpr_order[N];
    CODEGEN_ASSERT!(target.cat == CategoryX64::Reg);

    target.base = RegisterX64 {
      bits: (target.base.bits & RegisterX64::INDEX_MASK) | size as u8,
    };
    target.base
  }
}
