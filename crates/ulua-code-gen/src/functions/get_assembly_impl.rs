use alloc::{string::String, vec::Vec};
use core::{
  fmt::Arguments,
  mem::{size_of, take},
};

use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
use ulua_vm::{records::proto::Proto, type_aliases::t_value::TValue};

use crate::{
  enums::{
    code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags,
    function_stats_flags::FunctionStatsFlags,
  },
  functions::{
    assemble_helpers_a_64::assemble_helpers as assemble_helpers_a_64,
    assemble_helpers_x_64::assemble_helpers as assemble_helpers_x_64,
    context::gather_functions,
    dump::get_instruction_count,
    get_bytecode_type_name::UserdataTypes,
    log_function_header::log_function_header,
    log_function_types::log_function_types,
    lower_function::{lower_function_a_64, lower_function_x_64},
    proto_views::{code, name_bytes},
    with_lowering_stats::with_lowering_stats,
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64,
    assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions,
    function_bytecode_summary::FunctionBytecodeSummary,
    function_stats::FunctionStats,
    ir_builder::IrBuilder,
    lowering_stats::{FUNCTION_STATS_ENABLE, LoweringStats},
    module::ModuleHelpers,
  },
  traits::LogAppend,
};

impl LogAppend for AssemblyBuilderX64 {
  fn log_append(&mut self, args: Arguments<'_>) {
    self.log_append(args);
  }
}

impl LogAppend for AssemblyBuilderA64 {
  fn log_append(&mut self, args: Arguments<'_>) {
    self.log_append(args);
  }
}

/// 统一 X64/A64 汇编器反射接口，对齐上游 CodeGenAssembly.cpp 的
/// `template<typename AssemblyBuilder> getAssemblyImpl`
pub(crate) trait AsmBuilder: LogAppend {
  /// asm 计量步长：X64 按字节、A64 按 4 字节指令字
  const CODE_UNIT: u32;

  fn log_text(&self) -> bool;
  fn text_mut(&mut self) -> &mut String;
  fn finalize(&mut self) -> bool;
  fn get_code_size(&self) -> u32;
  fn get_instruction_count(&self) -> u32;

  /// 代码段字节长度（A64 为指令字数 × 4）
  fn code_len_bytes(&self) -> usize;
  /// 代码段按本机字节序追加到 `out`
  fn append_code_bytes(&self, out: &mut Vec<u8>);
  fn data_bytes(&self) -> &[u8];

  fn assemble_helpers(&mut self, helpers: &mut ModuleHelpers);

  /// lowering 主入口：IR → 机器码。
  fn lower_function(
    &mut self,
    ir: &mut IrBuilder,
    helpers: &mut ModuleHelpers,
    proto: Option<&Proto>,
    options: AssemblyOptions,
    stats: Option<&mut LoweringStats>,
  ) -> Result<(), CodeGenCompilationResult>;
}

impl AsmBuilder for AssemblyBuilderX64 {
  const CODE_UNIT: u32 = size_of::<u8>() as u32;

  fn log_text(&self) -> bool {
    self.log_text
  }

  fn text_mut(&mut self) -> &mut String {
    &mut self.text
  }

  fn finalize(&mut self) -> bool {
    self.finalize()
  }

  fn get_code_size(&self) -> u32 {
    self.get_code_size()
  }

  fn get_instruction_count(&self) -> u32 {
    // 原固有薄 getter `get_instruction_count` 字段化：直接读计数字段（可见性 pub(crate)）
    self.instruction_count
  }

  fn code_len_bytes(&self) -> usize {
    self.code.len()
  }

  fn append_code_bytes(&self, out: &mut Vec<u8>) {
    out.extend_from_slice(&self.code);
  }

  fn data_bytes(&self) -> &[u8] {
    &self.data
  }

  fn assemble_helpers(&mut self, helpers: &mut ModuleHelpers) {
    assemble_helpers_x_64(self, helpers)
  }

  /// lowering 主入口：IR → 机器码。
  fn lower_function(
    &mut self,
    ir: &mut IrBuilder,
    helpers: &mut ModuleHelpers,
    proto: Option<&Proto>,
    options: AssemblyOptions,
    stats: Option<&mut LoweringStats>,
  ) -> Result<(), CodeGenCompilationResult> {
    lower_function_x_64(ir, self, helpers, proto, options, stats)
  }
}

impl AsmBuilder for AssemblyBuilderA64 {
  const CODE_UNIT: u32 = size_of::<u32>() as u32;

  fn log_text(&self) -> bool {
    self.log_text
  }

  fn text_mut(&mut self) -> &mut String {
    &mut self.text
  }

  fn finalize(&mut self) -> bool {
    self.finalize()
  }

  fn get_code_size(&self) -> u32 {
    self.get_code_size()
  }

  fn get_instruction_count(&self) -> u32 {
    // A64 内核 code_pos 以指令字计步，指令数即代码规模（原固有转发方法删除，逐值等价）
    self.get_code_size()
  }

  fn code_len_bytes(&self) -> usize {
    self.code.len() * size_of::<u32>()
  }

  fn append_code_bytes(&self, out: &mut Vec<u8>) {
    // 指令字按本机字节序展开，等价于把 u32 切片重解释为字节
    out.extend(self.code.iter().flat_map(|word| word.to_ne_bytes()));
  }

  fn data_bytes(&self) -> &[u8] {
    &self.data
  }

  fn assemble_helpers(&mut self, helpers: &mut ModuleHelpers) {
    assemble_helpers_a_64(self, helpers)
  }

  /// lowering 主入口：IR → 机器码。
  fn lower_function(
    &mut self,
    ir: &mut IrBuilder,
    helpers: &mut ModuleHelpers,
    proto: Option<&Proto>,
    options: AssemblyOptions,
    stats: Option<&mut LoweringStats>,
  ) -> Result<(), CodeGenCompilationResult> {
    lower_function_a_64(ir, self, helpers, proto, options, stats)
  }
}

/// 单个函数的统计上报段，对齐 cpp CodeGenAssembly.cpp `getAssemblyImpl` 中
/// `if (stats && (stats->functionStatsFlags & FunctionStats_Enable)) {...}` 一节。
/// 可空 `stats` 的判空+解引用统一经 [`with_lowering_stats`] 门面完成；原型字段一律走
/// [`proto_views`] 的安全视图（review.md §2），本函数自身无裸指针解引用。
fn record_function_stats(
  mut stats: Option<&mut LoweringStats>,
  p: &Proto,
  root: &Proto,
  ir: &IrBuilder,
  asm_size: u32,
  asm_count: u32,
  code_unit: u32,
) {
  let Some(stats_flags) = with_lowering_stats(stats.as_deref_mut(), |s| s.function_stats_flags)
  else {
    return;
  };
  if stats_flags & FUNCTION_STATS_ENABLE == 0 {
    return;
  }

  // cpp：debugname 非空则取名，否则按 bytecodeid 是否等于顶层判「[top level]/[anonymous]」。
  // `name_bytes` 空指针即 None，与原 `!debugname.is_null()` 判据一致；`from_utf8_lossy` 复刻
  // 原 `cstr_cow(..).into_owned()`（C 串字节 → 宽容解码）。
  let function_stat_name = match name_bytes(p.debugname.cast_const(), p) {
    Some(name) => String::from_utf8_lossy(name).into_owned(),
    None if p.bytecodeid == root.bytecodeid => String::from("[top level]"),
    None => String::from("[anonymous]"),
  };

  let insns = code(p);

  let mut function_stat = FunctionStats {
    name: function_stat_name,
    ..Default::default()
  };
  function_stat.line = p.linedefined;
  function_stat.bcode_count = get_instruction_count(insns);
  function_stat.ir_count = ir.function.instructions.len() as u32;
  function_stat.asm_size = asm_size.wrapping_mul(code_unit);
  function_stat.asm_count = asm_count;

  if FunctionStatsFlags::FunctionStatsBytecodeSummary.is_set(stats_flags) {
    let summary = FunctionBytecodeSummary::from_proto(p, 0);
    function_stat
      .bytecode_summary
      .push(summary.get_counts(0).to_vec());
  }

  with_lowering_stats(stats, |s| s.functions.push(function_stat));
}

/// 泛型主体：X64/A64 共用的汇编输出流程
///
/// 返回值为机器码（`output_binary`）或 asm/IR 文本的字节表示，
/// 对应 cpp 侧承载二进制的 `std::string`。
fn get_assembly_impl<B: AsmBuilder>(
  build: &mut B,
  func: &TValue,
  options: AssemblyOptions,
  mut stats: Option<&mut LoweringStats>,
) -> Vec<u8> {
  // Safety: 契约保证 `func` 为 LClosure 型 `TValue`（栈位为 Lua 函数，见 get_assembly
  // 边界的 `lua_is_lfunction` 判据）：`as_closure` 的 tag 前提与 `inner.l` 的 union
  // 分支选择由此成立，`p` 指向 VM 在整段编译会话内持有的存活 root `Proto`。
  let root: &Proto = unsafe { &*func.as_closure().inner.l.p };

  if CodeGenFlags::CodeGenOnlyNativeModules.is_set(options.compilation_options.flags)
    // 短路顺序与 cpp `&&` 一致。
    && !LuauProtoFlag::LPF_NATIVE_MODULE.is_set(root.flags)
  {
    build.finalize();
    return Vec::new();
  }

  // cpp 侧 gatherFunctions 第三实参同样无条件求值。
  let root_is_native_function = root.flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8 != 0;

  // gather_functions 返回按 bytecodeid 索引的稀疏表（None 为空槽），沿原型链收集子 Proto。
  // 本处 flatten 依序紧凑（与原 `retain(!is_null)` 语义一致）。
  let protos: Vec<&Proto> = gather_functions(
    root,
    options.compilation_options.flags,
    root_is_native_function,
  )
  .into_iter()
  .flatten()
  .collect();

  with_lowering_stats(stats.as_deref_mut(), |s| {
    s.total_functions += protos.len() as u32
  });

  if protos.is_empty() {
    build.finalize();
    return Vec::new();
  }

  let mut helpers = ModuleHelpers::default();
  build.assemble_helpers(&mut helpers);

  if !options.include_outlined_code && options.include_assembly {
    build.text_mut().clear();
    build.log_append(format_args!(
      "; skipping {} bytes of outlined helpers\n",
      build.get_code_size().wrapping_mul(B::CODE_UNIT)
    ));
  }

  for &proto in &protos {
    let mut ir = IrBuilder::ir_builder_ir_builder(&options.compilation_options.hooks);

    ir.build_function_ir(proto);

    let mut asm_size = build.get_code_size();
    let mut asm_count = build.get_instruction_count();

    if options.include_assembly || options.include_ir {
      log_function_header(build, proto);
    }

    if options.include_ir_types {
      log_function_types(
        build,
        &ir.function,
        UserdataTypes::new(&options.compilation_options.userdata_types),
      );
    }

    // lower 失败原因此处不消费（cpp 同款丢弃），仅判成败
    let lower_failed = build
      .lower_function(
        &mut ir,
        &mut helpers,
        Some(proto),
        options.clone(),
        stats.as_deref_mut(),
      )
      .is_err();
    if lower_failed {
      if build.log_text() {
        build.log_append(format_args!("; skipping (can't lower)\n"));
      }

      asm_size = 0;
      asm_count = 0;

      with_lowering_stats(stats.as_deref_mut(), |s| s.skipped_functions += 1);
    } else {
      asm_size = build.get_code_size().wrapping_sub(asm_size);
      asm_count = build.get_instruction_count().wrapping_sub(asm_count);
    }

    // 此刻无并存 &mut 别名（lower 已完成对 stats 的使用），原型侧全为安全借用。
    record_function_stats(
      stats.as_deref_mut(),
      proto,
      root,
      &ir,
      asm_size,
      asm_count,
      B::CODE_UNIT,
    );

    if build.log_text() {
      build.log_append(format_args!("\n"));
    }
  }

  if !build.finalize() {
    return Vec::new();
  }

  if options.output_binary {
    let mut bytes = Vec::with_capacity(build.code_len_bytes() + build.data_bytes().len());
    build.append_code_bytes(&mut bytes);
    bytes.extend_from_slice(build.data_bytes());
    bytes
  } else {
    // builder 随本函数返回即 drop：take 免大字符串 clone
    take(build.text_mut()).into_bytes()
  }
}

/// X64 汇编输出入口（[`get_assembly`] 边界的契约在此已窄化为引用）。
pub(crate) fn get_assembly_impl_x_64(
  build: &mut AssemblyBuilderX64,
  func: &TValue,
  options: AssemblyOptions,
  stats: Option<&mut LoweringStats>,
) -> Vec<u8> {
  get_assembly_impl(build, func, options, stats)
}

/// A64 汇编输出入口（[`get_assembly`] 边界的契约在此已窄化为引用）。
pub(crate) fn get_assembly_impl_a_64(
  build: &mut AssemblyBuilderA64,
  func: &TValue,
  options: AssemblyOptions,
  stats: Option<&mut LoweringStats>,
) -> Vec<u8> {
  get_assembly_impl(build, func, options, stats)
}
