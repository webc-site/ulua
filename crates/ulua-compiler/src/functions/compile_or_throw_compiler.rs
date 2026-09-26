use core::ptr::NonNull;
use std::panic::panic_any;

use ulua_ast::{
  methods::ast_expr_function_ast_expr_function::AstExprFunctionArgs,
  records::{
    ast_expr_function::AstExprFunction,
    ast_name::AstName,
    ast_name_table::AstNameTable,
    ast_node::AstNode,
    ast_stat::AstStat,
    node_handle::{Node as ArenaNode, Nodes, OptNode},
    parse_errors::ParseErrors,
    parse_options::ParseOptions,
    parse_result::ParseResult,
  },
  visit::ast_stat_visit,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_proto_flag::LuauProtoFlag},
  macros::{luau_assert::LUAU_ASSERT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE},
};

use crate::{
  enums::global::Global,
  functions::{
    analyze_builtins::analyze_builtins, assign_mutable::assign_mutable,
    build_table_constant_map::build_table_constant_map, build_type_map::build_type_map,
    get_global_state::get_global_state, parse_pinned::parse_pinned,
    predict_table_shapes::predict_table_shapes, sref_compiler::sref_ast_name,
    track_values::track_values,
  },
  records::{
    compile_error::CompileError, compile_options::CompileOptions, compiler::Compiler,
    fenv_visitor::FenvVisitor, function_visitor::FunctionVisitor, node::Node,
    type_map_visitor::TypeMapVisitorArgs,
  },
};
pub fn compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
  bytecode: &mut BytecodeBuilder,
  parse_result: &ParseResult,
  names: &mut AstNameTable,
  input_options: &CompileOptions,
) {
  LUAU_TIMETRACE_SCOPE!("compileOrThrow", "Compiler");

  LUAU_ASSERT!(!parse_result.root.is_null());
  LUAU_ASSERT!(parse_result.errors.is_empty());

  let mut options = *input_options;
  let mut main_flags = 0u8;

  for hc in &parse_result.hotcomments {
    if hc.header {
      if let Some(value) = hc.content.strip_prefix("optimize ") {
        // cpp 用 `atoi`（Compiler.cpp:5589）：跳过前导空白、容忍尾部杂字符；
        // Rust `str::parse` 要求整串合法，`--!optimize 2 ` 会被误判为 0
        options.optimization_level = atoi_prefix(value).clamp(0, 2);
      }

      if hc.content == "native" {
        main_flags |= LuauProtoFlag::LPF_NATIVE_MODULE;
        options.set_for_native_compilation();
      }
    }
  }

  // cpp 的空根断言之后的 Rust 化收口：`ParseResult::root` 的契约是「只有解析报错
  // 才落空根」，而上方已断言 `errors` 为空，故空根属于使用契约被违反——以
  // `NonNull` 句柄取代「判空 + cast + 反复解引用」的裸指针三步样板，后续所有
  // 借用都从这一个类型化句柄派生。
  let root_block = NonNull::new(parse_result.root)
    .expect("compile requires a parse root; parse errors must be handled by the caller");
  let mut root_node: NonNull<AstNode> = root_block.cast::<AstNode>();
  // Safety: root_node 由上方 `NonNull` 证明非空，指向 ulua-parser 在 arena 分配的
  // `AstStatBlock`（本次编译结束前 arena 由调用方持有、地址稳定），repr(C) 首字段
  // 重合保证向上转 `AstNode` 基类合法；`&mut` 独占成立——此刻与各次再借用之间均为
  // 顺序使用，无并发别名。
  let root_ref = unsafe { root_node.as_mut() };
  // 根节点位置只用于报错：Copy 出栈，避免把 AST 引用跨语句持有。
  let root_location = root_ref.location;

  let mut functions = Vec::<Node<AstExprFunction>>::new();
  {
    let mut function_visitor = FunctionVisitor::new(&mut functions);
    // Safety: root_block 非空由上方 `NonNull` 证明；它指向 parser arena 中存活的
    // `AstStatBlock`（编译结束前 arena 由调用方持有），repr(C) 首字段重合保证
    // 向上转 `AstStat` 基类指针合法；FunctionVisitor 只收集子函数指针、不写 AST。
    unsafe {
      ast_stat_visit(root_block.cast::<AstStat>().as_ptr(), &mut function_visitor);
    }

    if function_visitor.has_native_function {
      options.set_for_native_compilation();
    }
  }

  let mut compiler = Compiler::new(bytecode, &options, names);

  assign_mutable(&mut compiler.globals, names, options.mutable_globals());
  track_values(
    &mut compiler.globals,
    &mut compiler.variables,
    &mut compiler.class_locals,
    root_ref,
  );

  let has_fenv = !names.get_str("getfenv").is_null() || !names.get_str("setfenv").is_null();
  if options.optimization_level >= 1 && has_fenv {
    let mut fenv_visitor = FenvVisitor::default();
    // Safety: 同上，root_block 非空且指向 arena 存活节点；FenvVisitor 仅置自身
    // bool 标志、不写 AST。
    unsafe {
      ast_stat_visit(root_block.cast::<AstStat>().as_ptr(), &mut fenv_visitor);
    }
    compiler.getfenv_used = fenv_visitor.getfenv_used;
    compiler.setfenv_used = fenv_visitor.setfenv_used;
  }

  if options.optimization_level >= 2 && !compiler.getfenv_used && !compiler.setfenv_used {
    compiler.builtins_fold = true;

    let math = names.get_str("math");
    if !math.is_null() && get_global_state(&compiler.globals, math) == Global::Default {
      compiler.builtins_fold_library_k = true;
    } else {
      for library in options.libraries_with_known_members() {
        let name = names.get_slice(library);
        if !name.is_null() && get_global_state(&compiler.globals, name) == Global::Default {
          compiler.builtins_fold_library_k = true;
          break;
        }
      }
    }
  }

  if options.optimization_level >= 1 {
    compiler.builtins = analyze_builtins(
      &compiler.globals,
      &compiler.variables,
      &options,
      root_ref,
      names,
    );

    compiler.table_constants = build_table_constant_map(&compiler.variables, root_ref);

    // 折叠只读遍历 AST，并以根节点地址作 map 键（地址句柄模型）。
    compiler.fold_constants(root_ref, false);

    compiler.table_shapes = predict_table_shapes(root_ref);
  }

  let mut userdata_count = 0usize;
  for name_bytes in options.userdata_types() {
    userdata_count += 1;
    let name = names.get_slice(name_bytes);
    if !name.is_null() {
      *compiler.userdata_types.get_or_insert(name) =
        bytecode.add_userdata_type(sref_ast_name(name)) as u8;
    }
  }

  if userdata_count
    > (LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_END.0
      - LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0) as usize
  {
    CompileError::raise(
      &root_location,
      format_args!("Exceeded userdata type limit in the compilation options"),
    );
  }

  if options.type_info_level >= 1 || options.optimization_level >= 2 {
    build_type_map(
      root_ref,
      TypeMapVisitorArgs {
        function_types: &mut compiler.function_types,
        local_types: &mut compiler.local_types,
        expr_types: &mut compiler.expr_types,
        host_vector_type: options.vector_type_bytes(),
        userdata_types: &compiler.userdata_types,
        builtin_types: &compiler.builtin_types,
        builtin_calls: &compiler.builtins,
        globals: &compiler.globals,
        library_member_type_cb: options.library_member_type_cb,
        bytecode,
      },
    );
  }

  for expr in functions {
    // expr 由 FunctionVisitor 在 arena 存活期内收集的节点句柄（存活契约见
    // `Node::borrow_mut`）；借用只覆盖本调用，compile_function 对其 body 子树只读遍历。
    let (_, protoflags) = compiler.compile_function(expr.borrow_mut(), 0);

    if LuauProtoFlag::LPF_NATIVE_FUNCTION.is_set(protoflags)
      && !LuauProtoFlag::LPF_NATIVE_MODULE.is_set(main_flags)
    {
      main_flags |= LuauProtoFlag::LPF_NATIVE_FUNCTION;
    }
  }

  // 合成「主函数」壳：全部字段都是栈上构造值（None / AstArray::default），
  // 唯一的外部接线是 `body` 指向 arena 存活的根块——已由 `root_block` 的
  // `NonNull` 证明非空，故此处无需 unsafe。
  let mut main = AstExprFunction::new(AstExprFunctionArgs {
    location: root_location,
    attributes: Nodes::default(),
    generics: Nodes::default(),
    generic_packs: Nodes::default(),
    self_: OptNode::default(),
    args: Nodes::default(),
    vararg: true,
    vararg_location: Default::default(),
    body: ArenaNode::from_non_null(root_block),
    function_depth: 0,
    debugname: AstName::default(),
    return_annotation: OptNode::default(),
    vararg_annotation: OptNode::default(),
    arg_location: None,
  });

  // main 是本函数栈上局部且直到函数末尾仍在作用域内，&mut 交接即存活证明；
  // compile_function 期间以其 body(=arena 存活的 root) 为语句树只读遍历，随后
  // functions.find(&main_ptr) 与 bytecode.set_main_function 均在本作用域结束前完成。
  let mainid = compiler.compile_function(&mut main, main_flags).0;

  let mainf = compiler.functions.find(&Node::from_mut(&mut main));
  LUAU_ASSERT!(mainf.is_some_and(|f| f.upvals.is_empty()));

  bytecode.set_main_function(mainid);
  bytecode.finalize();
}

/// C `atoi` 前缀语义：跳过前导空白，读可选符号与十进制数字前缀，尾部杂字符忽略；
/// 无数字得 0，溢出饱和。对应 cpp `atoi(hc.content.c_str() + 9)`。
fn atoi_prefix(s: &str) -> i32 {
  let bytes = s.as_bytes();
  let mut i = bytes
    .iter()
    .position(|&b| !b.is_ascii_whitespace())
    .unwrap_or(bytes.len());

  let neg = match bytes.get(i) {
    Some(b'-') => {
      i += 1;
      true
    }
    Some(b'+') => {
      i += 1;
      false
    }
    _ => false,
  };

  let mut v: i64 = bytes[i..]
    .iter()
    .copied()
    .take_while(|b| b.is_ascii_digit())
    .fold(0i64, |acc, b| {
      acc.saturating_mul(10).saturating_add(i64::from(b - b'0'))
    });
  v = if neg { -v } else { v };
  v.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

pub fn compile_or_throw_bytecode_builder_string_compile_options_parse_options<B>(
  bytecode: &mut BytecodeBuilder,
  source: &B,
  options: &CompileOptions,
  parse_options: &ParseOptions,
) where
  B: AsRef<[u8]> + ?Sized,
{
  let (_allocator, mut names, result) = parse_pinned(source, parse_options);

  if !result.errors.is_empty() {
    // 与 C++ `throw ParseErrors(result.errors)` 保真：panic 携带
    // ParseErrors payload（而非裸 "ParseErrors" 字符串），让捕获栈展开的
    // 调用方可以 downcast 并通过 Display 读取真实消息
    // （`ParseErrors::what()` = 单错误时取首条消息）。
    panic_any(ParseErrors::new(result.errors.clone()));
  }

  compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
    bytecode, &result, &mut names, options,
  );
}
