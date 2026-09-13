//! @interface-stub
use alloc::{string::String, vec::Vec};
use core::{
  ffi::CStr,
  mem::{size_of, size_of_val},
  ptr::null_mut,
};

use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
use ulua_vm::{
  functions::{lua_a_toobject::luaA_toobject, lua_is_lfunction::lua_is_lfunction},
  macros::{clvalue::clvalue, getstr::getstr},
  records::proto::Proto,
  type_aliases::instruction::Instruction,
};

#[cfg(target_arch = "aarch64")]
use crate::functions::assemble_helpers_code_gen_a_64::assemble_helpers as assemble_helpers_a_64;
#[cfg(not(target_arch = "aarch64"))]
use crate::functions::assemble_helpers_code_gen_x_64::assemble_helpers as assemble_helpers_x_64;
#[cfg(target_arch = "aarch64")]
use crate::functions::create_native_function::create_native_function_a_64;
#[cfg(not(target_arch = "aarch64"))]
use crate::functions::create_native_function::create_native_function_x_64;
#[cfg(target_arch = "aarch64")]
use crate::functions::get_cpu_features_a_64::get_cpu_features_a_64;
#[cfg(not(target_arch = "aarch64"))]
use crate::functions::get_cpu_features_x_64::get_cpu_features_x_64;
#[cfg(target_arch = "aarch64")]
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
#[cfg(not(target_arch = "aarch64"))]
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::{
  enums::{code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags},
  functions::{
    gather_functions::gather_functions, get_code_gen_context::get_code_gen_context,
    get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    compilation_options::CompilationOptions, compilation_result::CompilationResult,
    compilation_stats::CompilationStats, module_helpers::ModuleHelpers,
    proto_compilation_failure::ProtoCompilationFailure,
  },
  type_aliases::{
    lua_state::lua_State, module_id::ModuleId, native_proto_exec_data_ptr::NativeProtoExecDataPtr,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn compile_internal(
  module_id: &Option<ModuleId>,
  l: *mut lua_State,
  idx: i32,
  options: &CompilationOptions,
  stats: *mut CompilationStats,
) -> CompilationResult {
  unsafe {
    CODEGEN_ASSERT!(lua_is_lfunction(l, idx) != 0);
    let func = luaA_toobject(l, idx);

    let root: *mut Proto = (*clvalue!(func)).inner.l.p;

    if (options.flags & CodeGenFlags::CodeGenOnlyNativeModules as u32) != 0
      && ((*root).flags & LuauProtoFlag::LPF_NATIVE_MODULE as u8) == 0
      && ((*root).flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) == 0
    {
      return CompilationResult {
        result: CodeGenCompilationResult::NotNativeModule,
        proto_failures: Vec::new(),
      };
    }

    let code_gen_context = get_code_gen_context(l);
    if code_gen_context.is_null() {
      return CompilationResult {
        result: CodeGenCompilationResult::CodeGenNotInitialized,
        proto_failures: Vec::new(),
      };
    }

    let mut protos: Vec<*mut Proto> = Vec::new();
    gather_functions(
      &mut protos,
      root,
      options.flags,
      ((*root).flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) != 0,
    );

    protos.retain(|p| {
      let proto = *p;
      !proto.is_null() && (*proto).execdata.is_null()
    });

    if protos.is_empty() {
      return CompilationResult {
        result: CodeGenCompilationResult::NothingToCompile,
        proto_failures: Vec::new(),
      };
    }

    if !stats.is_null() {
      (*stats).functions_total = protos.len() as u32;
    }

    if let Some(module_id) = module_id.as_ref()
      && let Some(try_bind_existing_module_fn) = (*code_gen_context).try_bind_existing_module_fn
      && let Some(existing_module_bind_result) =
        try_bind_existing_module_fn(code_gen_context, module_id, &protos)
    {
      if !stats.is_null() {
        (*stats).functions_bound = existing_module_bind_result.functions_bound;
      }

      return CompilationResult {
        result: existing_module_bind_result.compilation_result,
        proto_failures: Vec::new(),
      };
    }

    #[cfg(target_arch = "aarch64")]
    let mut build = {
      let cpu_features = get_cpu_features_a_64();
      let mut build = AssemblyBuilderA64 {
        data: Vec::new(),
        code: Vec::new(),
        text: String::new(),
        log_text: false,
        features: 0,
        next_label: 1,
        pending_labels: Vec::new(),
        label_locations: Vec::new(),
        finalized: false,
        overflowed: false,
        data_pos: 0,
        code_pos: null_mut(),
        code_end: null_mut(),
      };
      build.assembly_builder_a_64_assembly_builder_a_64(false, cpu_features);
      build
    };

    #[cfg(not(target_arch = "aarch64"))]
    let mut build = {
      let cpu_features = get_cpu_features_x_64();
      AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, cpu_features)
    };

    let mut helpers = ModuleHelpers::default();

    #[cfg(target_arch = "aarch64")]
    assemble_helpers_a_64(&mut build, &mut helpers);

    #[cfg(not(target_arch = "aarch64"))]
    assemble_helpers_x_64(&mut build, &mut helpers);

    let mut compilation_result = CompilationResult::default();
    let mut native_protos: Vec<NativeProtoExecDataPtr> = Vec::with_capacity(protos.len());
    let mut total_ir_inst_count = 0u32;

    for proto in &protos {
      let mut proto_result = CodeGenCompilationResult::Success;

      let native_exec_data = {
        #[cfg(target_arch = "aarch64")]
        {
          create_native_function_a_64(
            &mut build,
            &mut helpers,
            *proto,
            &mut total_ir_inst_count,
            options,
            &mut proto_result,
          )
        }

        #[cfg(not(target_arch = "aarch64"))]
        {
          create_native_function_x_64(
            &mut build,
            &mut helpers,
            *proto,
            &mut total_ir_inst_count,
            options,
            &mut proto_result,
          )
        }
      };

      if let Some(native_exec_data) = native_exec_data {
        native_protos.push(native_exec_data);
      } else {
        compilation_result
          .proto_failures
          .push(ProtoCompilationFailure {
            result: proto_result,
            debugname: if !(*(*proto)).debugname.is_null() {
              let name = getstr((*(*proto)).debugname as *const _);
              CStr::from_ptr(name).to_string_lossy().into_owned()
            } else {
              String::new()
            },
            line: (*(*proto)).linedefined,
          });
      }
    }

    if !build.finalize() {
      compilation_result.result = CodeGenCompilationResult::CodeGenAssemblerFinalizationFailure;
      return compilation_result;
    }

    if native_protos.is_empty() {
      return compilation_result;
    }

    let native_code_size_bytes = size_of_val(build.code.as_slice());

    if !stats.is_null() {
      for native_exec_data in &native_protos {
        let header = &*get_native_proto_exec_data_header_mut(native_exec_data.as_ptr());

        (*stats).bytecode_size_bytes +=
          header.bytecode_instruction_count as usize * size_of::<Instruction>();
        (*stats).native_metadata_size_bytes +=
          header.bytecode_instruction_count as usize * size_of::<u32>();
      }

      (*stats).functions_compiled += native_protos.len() as u32;
      (*stats).native_code_size_bytes += native_code_size_bytes;
      (*stats).native_data_size_bytes += build.data.len();
    }

    for i in 0..native_protos.len() {
      let header = get_native_proto_exec_data_header_mut(native_protos[i].as_ptr());

      let begin = (*header).entry_offset_or_address as usize as u32;
      let end = if i + 1 < native_protos.len() {
        let next_header = get_native_proto_exec_data_header_mut(native_protos[i + 1].as_ptr());
        (*next_header).entry_offset_or_address as usize as u32
      } else {
        native_code_size_bytes as u32
      };

      CODEGEN_ASSERT!(begin < end);

      (*header).native_code_size = (end - begin) as usize;
    }

    let bind_result = ((*code_gen_context).bind_module_fn.unwrap())(
      code_gen_context,
      module_id,
      &protos,
      native_protos,
      build.data.as_ptr(),
      build.data.len(),
      build.code.as_ptr() as *const u8,
      native_code_size_bytes,
    );

    if !stats.is_null() {
      (*stats).functions_bound = bind_result.functions_bound;
    }

    if bind_result.compilation_result != CodeGenCompilationResult::Success {
      compilation_result.result = bind_result.compilation_result;
    }

    compilation_result
  }
}
