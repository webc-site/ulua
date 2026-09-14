use core::ptr::null_mut;

use ulua_common::FFlag::LuauCodegenFreeBlocks;

use crate::{
  enums::arch::Arch,
  functions::build_entry_function_code_gen_x_64::build_entry_function,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, base_code_gen_context::BaseCodeGenContext,
    unwind_builder::UnwindBuilder, unwind_builder_dwarf_2::UnwindBuilderDwarf2,
  },
};

pub fn init_header_functions(code_gen_context: &mut BaseCodeGenContext) -> bool {
  let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

  unsafe {
    start_info(&mut *code_gen_context.unwind_builder, Arch::X64);
  }

  let entry_locations =
    unsafe { build_entry_function(&mut build, &mut *code_gen_context.unwind_builder) };

  build.finalize();

  unsafe {
    finish_info(&mut *code_gen_context.unwind_builder);
  }

  CODEGEN_ASSERT!(build.data.is_empty());

  let mut code_start: *mut u8 = null_mut();

  if LuauCodegenFreeBlocks.get() {
    code_gen_context.gate_allocation_data = unsafe {
      code_gen_context.code_allocator.allocate(
        build.data.as_ptr(),
        build.data.len(),
        build.code.as_ptr(),
        build.code.len(),
      )
    };

    if code_gen_context.gate_allocation_data.start.is_null() {
      return false;
    }

    code_start = code_gen_context.gate_allocation_data.code_start;
  } else if !unsafe {
    code_gen_context.code_allocator.allocate_deprecated(
      build.data.as_ptr(),
      build.data.len(),
      build.code.as_ptr(),
      build.code.len(),
      &mut code_gen_context.gate_data_deprecated,
      &mut code_gen_context.gate_data_size_deprecated,
      &mut code_start,
    )
  } {
    return false;
  }

  unsafe {
    set_begin_offset(
      &mut *code_gen_context.unwind_builder,
      build.get_label_offset(&entry_locations.prologue_end) as usize,
    );

    code_gen_context.context.gate_entry =
      code_start.add(build.get_label_offset(&entry_locations.start) as usize);
    code_gen_context.context.gate_exit =
      code_start.add(build.get_label_offset(&entry_locations.epilogue_start) as usize);
  }

  true
}

#[cfg(target_os = "windows")]
unsafe fn start_info(unwind: &mut UnwindBuilder, arch: Arch) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder)
      .cast::<crate::records::unwind_builder_win::UnwindBuilderWin>())
      .start_info(arch);
  }
}

#[cfg(not(target_os = "windows"))]
unsafe fn start_info(unwind: &mut UnwindBuilder, arch: Arch) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder).cast::<UnwindBuilderDwarf2>()).start_info(arch);
  }
}

#[cfg(target_os = "windows")]
unsafe fn finish_info(unwind: &mut UnwindBuilder) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder)
      .cast::<crate::records::unwind_builder_win::UnwindBuilderWin>())
      .finish_info();
  }
}

#[cfg(not(target_os = "windows"))]
unsafe fn finish_info(unwind: &mut UnwindBuilder) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder).cast::<UnwindBuilderDwarf2>()).finish_info();
  }
}

#[cfg(target_os = "windows")]
unsafe fn set_begin_offset(unwind: &mut UnwindBuilder, begin_offset: usize) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder)
      .cast::<crate::records::unwind_builder_win::UnwindBuilderWin>())
      .set_begin_offset(begin_offset);
  }
}

#[cfg(not(target_os = "windows"))]
unsafe fn set_begin_offset(unwind: &mut UnwindBuilder, begin_offset: usize) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder).cast::<UnwindBuilderDwarf2>())
      .set_begin_offset(begin_offset);
  }
}
