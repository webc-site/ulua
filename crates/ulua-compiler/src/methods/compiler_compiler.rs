use core::ptr::{null, null_mut};

use ulua_ast::records::{
  ast_local::AstLocal, ast_name::AstName, ast_name_table::AstNameTable, location::Location,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  builtin_ast_types::BuiltinAstTypes, compile_options::CompileOptions, compiler::Compiler,
};

impl Compiler {
  pub fn new(
    bytecode: &mut BytecodeBuilder,
    options: &CompileOptions,
    names: &mut AstNameTable,
  ) -> Compiler {
    let export_name = unsafe { names.get_or_add_c_str(c"__EXP".as_ptr()) };

    let mut compiler = Compiler {
      bytecode: bytecode as *mut BytecodeBuilder,
      options: *options,
      functions: DenseHashMap::new(null_mut()),
      locals: DenseHashMap::new(null_mut()),
      globals: DenseHashMap::new(AstName::default()),
      variables: DenseHashMap::new(null_mut()),
      constants: DenseHashMap::new(null_mut()),
      locstants: DenseHashMap::new(null_mut()),
      table_constants: DenseHashMap::new(null_mut()),
      table_shapes: DenseHashMap::new(null_mut()),
      builtins: DenseHashMap::new(null_mut()),
      userdata_types: DenseHashMap::new(AstName::default()),
      function_types: DenseHashMap::new(null_mut()),
      local_types: DenseHashMap::new(null_mut()),
      expr_types: DenseHashMap::new(null_mut()),
      inline_builtins: DenseHashMap::new(null_mut()),
      inline_builtins_backup: DenseHashMap::new(null_mut()),
      expr_changes: Vec::new(),
      local_changes: Vec::new(),
      builtin_types: BuiltinAstTypes::new(options.vector_type),
      names: names as *mut AstNameTable,
      export_table_local: AstLocal::new(
        export_name,
        Location::default(),
        null_mut(),
        0,
        0,
        null_mut(),
        true,
      ),
      builtins_fold: null(),
      builtins_fold_library_k: false,
      reg_top: 0,
      stack_size: 0,
      arg_count: 0,
      has_loops: false,
      current_function: null_mut(),
      block_depth: 0,
      getfenv_used: false,
      setfenv_used: false,
      local_stack: Vec::new(),
      upvals: Vec::new(),
      loop_jumps: Vec::new(),
      loops: Vec::new(),
      inline_frames: Vec::new(),
      captures: Vec::new(),
      exported_locals: Vec::new(),
      exported_classes: Vec::new(),
      class_locals: DenseHashMap::new(AstName::default()),
    };

    compiler.local_stack.reserve(16);
    compiler.upvals.reserve(16);
    compiler
  }
}

pub fn compiler_compiler(
  bytecode: &mut BytecodeBuilder,
  options: &CompileOptions,
  names: &mut AstNameTable,
) -> Compiler {
  Compiler::new(bytecode, options, names)
}
