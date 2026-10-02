use alloc::{string::String, vec::Vec};
use core::{
  iter::once,
  mem::{size_of, size_of_val},
};

use ulua_common::{enums::luau_proto_flag::LuauProtoFlag, functions::c_str::cstr_cow};
use ulua_vm::{
  functions::{lua_a_toobject::lua_a_toobject, lua_is_lfunction::lua_is_lfunction},
  macros::getstr::getstr,
  records::proto::Proto,
  type_aliases::instruction::Instruction,
};

#[cfg(target_arch = "aarch64")]
use crate::functions::assemble_helpers_a_64::assemble_helpers as assemble_helpers_a_64;
#[cfg(not(target_arch = "aarch64"))]
use crate::functions::assemble_helpers_x_64::assemble_helpers as assemble_helpers_x_64;
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
    context::gather_functions, get_code_gen_context::get_code_gen_context,
    with_compilation_stats::with_compilation_stats,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    compilation_options::CompilationOptions,
    compilation_result::{CompilationResult, ProtoCompilationFailure},
    compilation_stats::CompilationStats,
    module::ModuleHelpers,
    native_proto_exec_data_header::NativeProtoExecDataHeader,
  },
  traits::HeaderEntryBuilder,
  type_aliases::{
    api::LuaState,
    module_id::ModuleId,
    native_proto_exec_data_ptr::{NativeProtoExecDataHeaderExt, NativeProtoExecDataPtr},
  },
};

/// 从编译失败的 proto 上摘取诊断字段：`debugname` 深拷贝为 `String`、`linedefined` 原样取。
///
/// # Safety
/// `proto` 必须指向存活的 `Proto`，其 `debugname` 为 null 或指向存活且以 NUL 结尾的
/// `TString`（两者均由 `gather_functions` 的输出契约保证）。
unsafe fn proto_failure_info(proto: *mut Proto) -> (String, i32) {
  // Safety: 依 `# Safety` 契约，`proto` 存活，此处一次性读出诊断字段避免多处重复解引用。
  let (debugname, line) = unsafe { ((*proto).debugname, (*proto).linedefined) };

  let name = if debugname.is_null() {
    String::new()
  } else {
    // Safety: 同上契约，`debugname` 指向存活 TString；`getstr` 只取该串字符区首指针。
    let chars = unsafe { getstr(debugname as *const _) };
    // Safety: `chars` 为 Lua 字符串恒 NUL 结尾的存活字节序列首指针，门面按 NUL 截断读取
    // 在克隆为 String 前完成整串扫描。
    unsafe { cstr_cow(chars) }.into_owned()
  };

  (name, line)
}

/// 在短临时借用上访问某个 native proto 的 execdata header（统计与代码段回填共用）。
///
/// 不变量：`exec_data` 来自本轮 `create_native_function_*` 的成功返回，直到
/// `bind_module` 交接前一直存活；header 视图经 [`NativeProtoExecDataHeaderExt::header_mut`]
/// 门面派生（unsafe 收口见该 trait 契约），借用只在闭包执行期间存在。
fn with_proto_header<R>(
  exec_data: &mut NativeProtoExecDataPtr,
  f: impl FnOnce(&mut NativeProtoExecDataHeader) -> R,
) -> R {
  f(exec_data.header_mut())
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn compile_internal(
  module_id: &Option<ModuleId>,
  l: *mut LuaState,
  idx: i32,
  options: &CompilationOptions,
  mut stats: Option<&mut CompilationStats>,
) -> CompilationResult {
  // 本函数的 `unsafe` 逐处就地标注：`l`/`idx` 的 Lua C-ABI 读栈、`root`/`protos` 的
  // `Proto*` 解引用、`code_gen_context` 的上下文解引用与其上 C ABI 回调、execdata header
  // 回填，均为最小封装的 (b) 类边界；`stats` 的可空裸指针由 `with_compilation_stats`
  // 门面统一收口，本函数体内不再出现 `(*stats)` 解引用。

  // Safety: 契约保证 `l` 为存活 LuaState*、`idx` 为界内栈位，`lua_is_lfunction` 与
  // `lua_a_toobject` 按 Lua C-ABI 读取该栈位。
  unsafe {
    CODEGEN_ASSERT!(lua_is_lfunction(l, idx) != 0);
  }
  // Safety: 同上，且上一条断言确认该栈位是 Lua 闭包，`as_closure` 的 TValue→GCObject→
  // Closure 解引用链由此前提保证；`inner.l.p` 为构造期接线的非空 root Proto*。
  let root: *mut Proto = unsafe { (*lua_a_toobject(&*l, idx)).as_closure().inner.l.p };
  // Safety: `root` 为存活 Proto*（C-ABI 契约），此处只取 flags 快照供后续判定，
  // 避免整段流程反复解引用裸指针。
  let root_flags = unsafe { (*root).flags };

  if CodeGenFlags::CodeGenOnlyNativeModules.is_set(options.flags)
    && !LuauProtoFlag::LPF_NATIVE_MODULE.is_set(root_flags)
    && !LuauProtoFlag::LPF_NATIVE_FUNCTION.is_set(root_flags)
  {
    return CompilationResult {
      result: CodeGenCompilationResult::NotNativeModule,
      proto_failures: Vec::new(),
    };
  }

  // Safety: `get_code_gen_context` 依其 `# Safety` 契约读 `l->global->ecb.context` 并
  // 返回串行窗口内的独占借用（本回调路径无重入编译，借用持续到函数尾，中途不再
  // 出现裸指针重建），None 即未初始化、原样早退。
  let Some(code_gen_context) = (unsafe { get_code_gen_context(l) }) else {
    return CompilationResult {
      result: CodeGenCompilationResult::CodeGenNotInitialized,
      proto_failures: Vec::new(),
    };
  };

  // Safety: `gather_functions` 依其 `# Safety` 契约消费存活的 `root`，返回的稀疏表元素
  // 是同批存活的 `Proto*`（含递归收集的后代），`execdata` 判空只读这些存活对象。
  let protos: Vec<*mut Proto> = unsafe {
    gather_functions(
      root,
      options.flags,
      (root_flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) != 0,
    )
    .into_iter()
    .flatten()
    // J1 Phase 2b：force_recompile（暖重编译）时保留已编译 proto——重编译并重绑定；
    // 常规路径跳过已编译（NothingToCompile 语义）。
    .filter(|&proto| options.force_recompile || (*proto).execdata.is_null())
    .collect()
  };

  if protos.is_empty() {
    return CompilationResult {
      result: CodeGenCompilationResult::NothingToCompile,
      proto_failures: Vec::new(),
    };
  }

  with_compilation_stats(stats.as_deref_mut(), |stats| {
    stats.functions_total = protos.len() as u32
  });

  // `try_bind_existing_module` 内部按 kind 静态分支（Standalone 恒 None），
  // `module_id` 来自上层调用方的存活借用，`protos` 元素存活。
  let existing_module_bind_result = match module_id.as_ref() {
    Some(module_id) => code_gen_context.try_bind_existing_module(module_id, &protos),
    None => None,
  };

  if let Some(existing_module_bind_result) = existing_module_bind_result {
    with_compilation_stats(stats.as_deref_mut(), |stats| {
      stats.functions_bound = existing_module_bind_result.functions_bound
    });

    return CompilationResult {
      result: existing_module_bind_result.compilation_result,
      proto_failures: Vec::new(),
    };
  }

  #[cfg(target_arch = "aarch64")]
  let mut build = {
    let cpu_features = get_cpu_features_a_64();
    // 复用构造函数，避免逐字段字面量初始化
    AssemblyBuilderA64::new(false, cpu_features)
  };

  #[cfg(not(target_arch = "aarch64"))]
  let mut build = {
    let cpu_features = get_cpu_features_x_64();
    AssemblyBuilderX64::new(false, cpu_features)
  };

  let mut helpers = ModuleHelpers::default();

  #[cfg(target_arch = "aarch64")]
  assemble_helpers_a_64(&mut build, &mut helpers);

  #[cfg(not(target_arch = "aarch64"))]
  assemble_helpers_x_64(&mut build, &mut helpers);

  let mut compilation_result = CompilationResult::default();
  let mut native_protos: Vec<NativeProtoExecDataPtr> = Vec::with_capacity(protos.len());
  let mut total_ir_inst_count = 0u32;

  for &proto in &protos {
    // 出参改返回值：成败与失败原因合并进 Result
    let created = {
      #[cfg(target_arch = "aarch64")]
      {
        // Safety: `proto` 为 `gather_functions` 输出的存活 Proto*，build/helpers 为本函数
        // 局部活借用，符合 `create_native_function_a_64` 的 `# Safety` 前置条件。
        unsafe {
          create_native_function_a_64(
            &mut build,
            &mut helpers,
            proto,
            &mut total_ir_inst_count,
            options,
          )
        }
      }

      #[cfg(not(target_arch = "aarch64"))]
      {
        // Safety: 同上（X64 分支），`proto` 与三个借用均存活。
        unsafe {
          create_native_function_x_64(
            &mut build,
            &mut helpers,
            proto,
            &mut total_ir_inst_count,
            options,
          )
        }
      }
    };

    match created {
      Ok(native_exec_data) => native_protos.push(native_exec_data),
      Err(proto_result) => {
        // Safety: `proto` 存活，其 `debugname` 满足 `proto_failure_info` 的前置条件。
        let (debugname, line) = unsafe { proto_failure_info(proto) };

        compilation_result
          .proto_failures
          .push(ProtoCompilationFailure {
            result: proto_result,
            debugname,
            line,
          });
      }
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

  // 统计口径不变：先按 header 汇总字节码/元数据大小，再一次性写进可空 stats 门面。
  let mut bytecode_size_bytes = 0usize;
  let mut native_metadata_size_bytes = 0usize;

  for native_exec_data in &native_protos {
    let instruction_count = native_exec_data.header().bytecode_instruction_count;

    bytecode_size_bytes += instruction_count as usize * size_of::<Instruction>();
    native_metadata_size_bytes += instruction_count as usize * size_of::<u32>();
  }

  with_compilation_stats(stats.as_deref_mut(), |stats| {
    stats.bytecode_size_bytes += bytecode_size_bytes;
    stats.native_metadata_size_bytes += native_metadata_size_bytes;
    stats.functions_compiled += native_protos.len() as u32;
    stats.native_code_size_bytes += native_code_size_bytes;
    stats.native_data_size_bytes += build.data.len();
  });

  // 回填每个 proto 的原生代码段大小：end 取下一 proto 的入口偏移，末位取代码总长。
  // 前置条件与原 `CODEGEN_ASSERT!(begin < end)` 一致：入口偏移随链接顺序严格递增。
  let ends: Vec<u32> = native_protos
    .iter()
    .skip(1)
    .map(|p| p.header().entry_offset_or_address as usize as u32)
    .chain(once(native_code_size_bytes as u32))
    .collect();

  for (proto, end) in native_protos.iter_mut().zip(ends) {
    let begin = proto.header().entry_offset_or_address as usize as u32;
    CODEGEN_ASSERT!(begin < end);
    with_proto_header(proto, |header| {
      header.native_code_size = (end - begin) as usize;
    });
  }

  // `bind_module` 已切片化（data/code 取自本帧局部落地值 `build` 的产出缓冲，code 段经
  // `code_bytes()` 字节视图，长度与 `native_code_size_bytes` 逐值一致），内部按 kind 静态
  // 分支取代原 fn 槽分派；借用沿用上方派生的唯一 `&mut`。
  let bind_result = code_gen_context.bind_module(
    module_id,
    &protos,
    native_protos,
    &build.data,
    build.code_bytes(),
  );

  with_compilation_stats(stats, |stats| {
    stats.functions_bound = bind_result.functions_bound
  });

  if bind_result.compilation_result != CodeGenCompilationResult::Success {
    compilation_result.result = bind_result.compilation_result;
  }

  compilation_result
}
