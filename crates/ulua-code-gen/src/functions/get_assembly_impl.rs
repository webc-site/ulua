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
    assemble_helpers_code_gen_a_64::assemble_helpers as assemble_helpers_a_64,
    assemble_helpers_code_gen_x_64::assemble_helpers as assemble_helpers_x_64,
    gather_functions::gather_functions,
    get_bytecode_type_name::UserdataTypes,
    get_instruction_count_code_gen_assembly::get_instruction_count,
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
    module_helpers::ModuleHelpers,
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

  /// # Safety
  /// proto 指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  unsafe fn lower_function(
    &mut self,
    ir: &mut IrBuilder,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
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
    self.get_instruction_count()
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
  /// # Safety
  /// `proto`/`stats` 指针必须有效且指向存活对象，`ir` 与 proto 一致，调用方须满足 C++ 参考实现的前置条件。
  unsafe fn lower_function(
    &mut self,
    ir: &mut IrBuilder,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
  ) -> Result<(), CodeGenCompilationResult> {
    // Safety: 本 trait 方法的 `# Safety` 契约（`proto`/`stats` 存活、`ir` 与 proto 一致）原样
    // 透传给 `lower_function_x_64`，参数一字不改，前置条件与调用本方法时相同。
    unsafe { lower_function_x_64(ir, self, helpers, proto, options, stats) }
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
    self.get_instruction_count()
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
  /// # Safety
  /// `proto`/`stats` 指针必须有效且指向存活对象，`ir` 与 proto 一致，调用方须满足 C++ 参考实现的前置条件。
  unsafe fn lower_function(
    &mut self,
    ir: &mut IrBuilder,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
  ) -> Result<(), CodeGenCompilationResult> {
    // Safety: 同 X64 版——本 trait 方法的 `# Safety` 契约原样透传给 `lower_function_a_64`，
    // 参数一字不改，前置条件与调用本方法时相同。
    unsafe { lower_function_a_64(ir, self, helpers, proto, options, stats) }
  }
}

/// 单个函数的统计上报段，对齐 cpp CodeGenAssembly.cpp `getAssemblyImpl` 中
/// `if (stats && (stats->functionStatsFlags & FunctionStats_Enable)) {...}` 一节。
/// 可空 `stats` 的判空+解引用统一经 [`with_lowering_stats`] 门面完成；原型字段一律走
/// [`proto_views`] 的安全视图（review.md §2），本函数自身无裸指针解引用。
fn record_function_stats(
  stats: *mut LoweringStats,
  p: &Proto,
  root: &Proto,
  ir: &IrBuilder,
  asm_size: u32,
  asm_count: u32,
  code_unit: u32,
) {
  let Some(stats_flags) = with_lowering_stats(stats, |s| s.function_stats_flags) else {
    return;
  };
  if stats_flags & FUNCTION_STATS_ENABLE == 0 {
    return;
  }

  // cpp：debugname 非空则取名，否则按 bytecodeid 是否等于顶层判「[top level]/[anonymous]」。
  // `name_bytes` 空指针即 None，与原 `!debugname.is_null()` 判据一致；`from_utf8_lossy` 复刻
  // 原 `cstr_cow(..).into_owned()`（CStr 字节 → 宽容解码）。
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
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
unsafe fn get_assembly_impl<B: AsmBuilder>(
  build: &mut B,
  func: *const TValue,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> Vec<u8> {
  // Safety: 依函数契约，`func` 为存活且含闭包值的 `TValue`，`as_closure` 得到存活 Closure，
  // 其 `inner.l.p` 即存活 root `Proto`；本块只取指针副本，不解引用。
  let root: *mut Proto = unsafe { (*func).as_closure().inner.l.p };

  // 反射/转储路径只读原型：对 `Proto` 的写入只发生在 bind_native_protos/on_disable/
  // on_destroy_function 等编译阶段，与本函数无交叠，故此处一次派生共享引用、后续一律走
  // 具名字段读取（review.md §2）。
  // Safety: `root` 依上契约存活且本段无并存可变借用。
  let root_ref = unsafe { &*root };

  if CodeGenFlags::CodeGenOnlyNativeModules.is_set(options.compilation_options.flags)
    // 短路顺序与 cpp `&&` 一致。
    && !LuauProtoFlag::LPF_NATIVE_MODULE.is_set(root_ref.flags)
  {
    build.finalize();
    return Vec::new();
  }

  // cpp 侧 gatherFunctions 第三实参同样无条件求值。
  let root_is_native_function = root_ref.flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8 != 0;

  // gather_functions 返回按 bytecodeid 索引的稀疏表（None 为空槽），沿原型链收集子 Proto。
  // 本处 flatten 依序紧凑（与原 `retain(!is_null)` 语义一致）。
  // Safety: `root` 指向存活 Proto，gather_functions 按其自身契约沿原型链只读遍历。
  let protos: Vec<*mut Proto> = unsafe {
    gather_functions(
      root,
      options.compilation_options.flags,
      root_is_native_function,
    )
  }
  .into_iter()
  .flatten()
  .collect();

  with_lowering_stats(stats, |s| s.total_functions += protos.len() as u32);

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

  for p in protos {
    // Safety: `p` 为 gather_functions 收集的非空存活 Proto（None 槽已被上面的 flatten 去掉）；
    // 本轮迭代只读取其字段，写原型的路径不在本函数调用栈上。
    let proto = unsafe { &*p };

    let mut ir = IrBuilder::ir_builder_ir_builder(&options.compilation_options.hooks);

    // Safety: `p` 与 `proto` 同源，build_function_ir 按其契约只读原型字段构建 IR。
    unsafe { ir.build_function_ir(p) };

    let mut asm_size = build.get_code_size();
    let mut asm_count = build.get_instruction_count();

    if options.include_assembly || options.include_ir {
      log_function_header(build, proto);
    }

    if options.include_ir_types {
      // log_function_types 只读已构建好的 `ir.function`（全为安全借用），本身即安全 `fn`。
      log_function_types(
        build,
        &ir.function,
        UserdataTypes::new(&options.compilation_options.userdata_types),
      );
    }

    // lower 失败原因此处不消费（cpp 同款丢弃），仅判成败
    // Safety: `p`/`stats` 存活性与 `ir` 与 `p` 的一致性由本函数 `# Safety` 契约保证，
    // trait 方法将契约透传给平台 lower_function_{x64,a64}。
    let lower_failed =
      unsafe { build.lower_function(&mut ir, &mut helpers, p, options.clone(), stats) }.is_err();
    if lower_failed {
      if build.log_text() {
        build.log_append(format_args!("; skipping (can't lower)\n"));
      }

      asm_size = 0;
      asm_count = 0;

      with_lowering_stats(stats, |s| s.skipped_functions += 1);
    } else {
      asm_size = build.get_code_size().wrapping_sub(asm_size);
      asm_count = build.get_instruction_count().wrapping_sub(asm_count);
    }

    // Safety: `stats` 为可空裸指针，`with_lowering_stats` 门面内部判空；此刻无并存
    // &mut 别名（单线程串行，lower 已完成对 stats 的使用）。原型侧全为安全借用。
    record_function_stats(
      stats,
      proto,
      root_ref,
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

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn get_assembly_impl_x_64(
  build: &mut AssemblyBuilderX64,
  func: *const TValue,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> Vec<u8> {
  // Safety: 本 `unsafe fn` 将自身 `# Safety` 契约（`func`/`stats` 存活、参数合法）原样透传给泛型
  // `get_assembly_impl`，参数一字不改。
  unsafe { get_assembly_impl(build, func, options, stats) }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn get_assembly_impl_a_64(
  build: &mut AssemblyBuilderA64,
  func: *const TValue,
  options: AssemblyOptions,
  stats: *mut LoweringStats,
) -> Vec<u8> {
  // Safety: 同 X64 入口——将 `# Safety` 契约原样透传给泛型 `get_assembly_impl`，参数一字不改。
  unsafe { get_assembly_impl(build, func, options, stats) }
}
