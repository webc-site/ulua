use core::ptr::null_mut;

use ulua_ast::{
  methods::ast_expr_function_ast_expr_function::AstExprFunctionArgs,
  records::{
    ast_array::AstArray, ast_expr_function::AstExprFunction, ast_name::AstName,
    ast_name_table::AstNameTable, ast_node::AstNode, ast_stat::AstStat, parse_result::ParseResult,
  },
  visit::ast_stat_visit,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  FFlag,
  enums::{
    luau_bytecode_type::{LBC_TYPE_TAGGED_USERDATA_BASE, LBC_TYPE_TAGGED_USERDATA_END},
    luau_proto_flag::LuauProtoFlag,
  },
  macros::{luau_assert::LUAU_ASSERT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE},
};

use crate::{
  enums::global::Global,
  functions::{
    analyze_builtins::analyze_builtins, assign_mutable::assign_mutable,
    build_table_constant_map::build_table_constant_map, build_type_map::build_type_map,
    get_global_state::get_global_state, predict_table_shapes::predict_table_shapes,
    set_compile_options_for_native_compilation::set_compile_options_for_native_compilation,
    track_values::track_values,
  },
  records::{
    compile_error::CompileError, compile_options::CompileOptions, compiler::Compiler,
    fenv_visitor::FenvVisitor, function_visitor::FunctionVisitor,
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
        let level = value.parse::<i32>().unwrap_or(0).clamp(0, 2);
        options.optimization_level = level;
      }

      if hc.content == "native" {
        main_flags |= LuauProtoFlag::LPF_NATIVE_MODULE as u8;
        set_compile_options_for_native_compilation(&mut options);
      }
    }
  }

  let root = parse_result.root;
  let root_node = root as *mut AstNode;

  let mut functions = Vec::<*mut AstExprFunction>::new();
  {
    let mut function_visitor = FunctionVisitor::new(&mut functions);
    unsafe {
      ast_stat_visit(root as *mut AstStat, &mut function_visitor);
    }

    if function_visitor.has_native_function {
      set_compile_options_for_native_compilation(&mut options);
    }
  }

  let mut compiler = Compiler::new(bytecode, &options, names);

  assign_mutable(&mut compiler.globals, names, options.mutable_globals);
  unsafe {
    track_values(
      &mut compiler.globals,
      &mut compiler.variables,
      &mut compiler.class_locals,
      root_node,
    )
  };

  let has_fenv = unsafe {
    !names.get(c"getfenv".as_ptr()).value.is_null()
      || !names.get(c"setfenv".as_ptr()).value.is_null()
  };
  if options.optimization_level >= 1 && has_fenv {
    let mut fenv_visitor = FenvVisitor::new(&mut compiler.getfenv_used, &mut compiler.setfenv_used);
    unsafe {
      ast_stat_visit(root as *mut AstStat, &mut fenv_visitor);
    }
  }

  if options.optimization_level >= 2 && !compiler.getfenv_used && !compiler.setfenv_used {
    compiler.builtins_fold = &compiler.builtins as *const _;

    let math = unsafe { names.get(c"math".as_ptr()) };
    if !math.value.is_null() && get_global_state(&compiler.globals, math) == Global::Default {
      compiler.builtins_fold_library_k = true;
    } else if !options.libraries_with_known_members.is_null() {
      unsafe {
        let mut ptr = options.libraries_with_known_members;
        while !(*ptr).is_null() {
          let name = names.get(*ptr);
          if !name.value.is_null() && get_global_state(&compiler.globals, name) == Global::Default {
            compiler.builtins_fold_library_k = true;
            break;
          }
          ptr = ptr.add(1);
        }
      }
    }
  }

  if options.optimization_level >= 1 {
    unsafe {
      analyze_builtins(
        &mut compiler.builtins,
        &compiler.globals,
        &compiler.variables,
        &options,
        root_node,
        names,
      )
    };

    if FFlag::LuauCompilePropagateTableProps2.get() && FFlag::LuauCompileFoldOptimize.get() {
      unsafe {
        build_table_constant_map(
          &mut compiler.table_constants,
          &compiler.variables,
          root_node,
        )
      };
    }

    unsafe {
      compiler.fold_constants(root_node, false);
    };

    unsafe { predict_table_shapes(&mut compiler.table_shapes, root_node) };
  }

  if !options.userdata_types.is_null() {
    unsafe {
      let mut ptr = options.userdata_types;
      while !(*ptr).is_null() {
        let name = names.get(*ptr);
        if !name.is_null() {
          *compiler.userdata_types.get_or_insert(name) =
            bytecode.add_userdata_type(name.as_str_or_empty()) as u8;
        }
        ptr = ptr.add(1);
      }

      let count = ptr.offset_from(options.userdata_types) as u16;
      if count > (LBC_TYPE_TAGGED_USERDATA_END.0 - LBC_TYPE_TAGGED_USERDATA_BASE.0) {
        CompileError::raise(
          &(*root).base.base.location,
          format_args!("Exceeded userdata type limit in the compilation options"),
        );
      }
    }
  }

  if options.type_info_level >= 1 || options.optimization_level >= 2 {
    unsafe {
      build_type_map(
        root_node,
        TypeMapVisitorArgs {
          function_types: &mut compiler.function_types,
          local_types: &mut compiler.local_types,
          expr_types: &mut compiler.expr_types,
          host_vector_type: options.vector_type,
          userdata_types: &compiler.userdata_types,
          builtin_types: &compiler.builtin_types,
          builtin_calls: &compiler.builtins,
          globals: &compiler.globals,
          library_member_type_cb: options.library_member_type_cb,
          bytecode,
        },
      )
    };
  }

  for expr in functions {
    let mut protoflags = 0u8;
    unsafe { compiler.compile_function(expr, &mut protoflags) };

    if (protoflags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) != 0
      && (main_flags & LuauProtoFlag::LPF_NATIVE_MODULE as u8) == 0
    {
      main_flags |= LuauProtoFlag::LPF_NATIVE_FUNCTION as u8;
    }
  }

  let mut main = unsafe {
    AstExprFunction::new(AstExprFunctionArgs {
      location: (*root).base.base.location,
      attributes: AstArray::default(),
      generics: AstArray::default(),
      generic_packs: AstArray::default(),
      self_: null_mut(),
      args: AstArray::default(),
      vararg: true,
      vararg_location: Default::default(),
      body: root,
      function_depth: 0,
      debugname: AstName::default(),
      return_annotation: null_mut(),
      vararg_annotation: null_mut(),
      arg_location: None,
    })
  };

  let mainid = unsafe { compiler.compile_function(&mut main, &mut main_flags) };

  let main_ptr = &mut main as *mut AstExprFunction;
  let mainf = compiler.functions.find(&main_ptr);
  LUAU_ASSERT!(mainf.is_some_and(|f| f.upvals.is_empty()));

  bytecode.set_main_function(mainid);
  bytecode.finalize();
}
