use alloc::{string::String, vec::Vec};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
  ast_expr_table::AstExprTable, ast_local::AstLocal, ast_name::AstName,
  ast_name_table::AstNameTable,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType, records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::{global::Global, table_constant_kind::TableConstantKind},
  records::{
    builtin_ast_types::BuiltinAstTypes, capture::Capture, compile_options::CompileOptions,
    constant::Constant, expr_constant_change::ExprConstantChange, function::Function,
    inline_frame::InlineFrame, local::Local, local_constant_change::LocalConstantChange,
    r#loop::Loop, loop_jump::LoopJump, table_shape::TableShape, variable::Variable,
  },
};

/// AD 指令 16 位 D 域能编码的最大常量索引（C++ `0x8000`，超限走慢路径）
pub(crate) const K_MAX_AD_INDEX: i32 = 0x8000;

/// import id 10 位域上限（C++ `1024`）
pub(crate) const K_MAX_IMPORT_ID: i32 = 1024;

/// GETIMPORT 辅助字的高位标志（C++ `0x80000000`，GETIMPORT flip）
pub(crate) const K_GETIMPORT_FLAG: u32 = 0x80000000;

#[derive(Debug)]
pub struct Compiler {
  pub bytecode: *mut BytecodeBuilder,
  pub options: CompileOptions,
  pub functions: DenseHashMap<*mut AstExprFunction, Function>,
  pub locals: DenseHashMap<*mut AstLocal, Local>,
  pub globals: DenseHashMap<AstName, Global>,
  pub variables: DenseHashMap<*mut AstLocal, Variable>,
  pub constants: DenseHashMap<*mut AstExpr, Constant>,
  pub locstants: DenseHashMap<*mut AstLocal, Constant>,
  pub table_constants: DenseHashMap<*mut AstLocal, TableConstantKind>,
  pub table_shapes: DenseHashMap<*mut AstExprTable, TableShape>,
  pub builtins: DenseHashMap<*mut AstExprCall, i32>,
  pub userdata_types: DenseHashMap<AstName, u8>,
  pub function_types: DenseHashMap<*mut AstExprFunction, String>,
  pub local_types: DenseHashMap<*mut AstLocal, LuauBytecodeType>,
  pub expr_types: DenseHashMap<*mut AstExpr, LuauBytecodeType>,
  pub inline_builtins: DenseHashMap<*mut AstExprCall, i32>,
  pub inline_builtins_backup: DenseHashMap<*mut AstExprCall, i32>,
  pub expr_changes: Vec<ExprConstantChange>,
  pub local_changes: Vec<LocalConstantChange>,
  pub builtin_types: BuiltinAstTypes,
  pub names: *mut AstNameTable,
  pub export_table_local: AstLocal,
  pub builtins_fold: *const DenseHashMap<*mut AstExprCall, i32>,
  pub builtins_fold_library_k: bool,
  pub reg_top: u32,
  pub stack_size: u32,
  pub arg_count: usize,
  pub has_loops: bool,
  pub current_function: *mut AstExprFunction,
  pub block_depth: usize,
  pub getfenv_used: bool,
  pub setfenv_used: bool,
  pub local_stack: Vec<*mut AstLocal>,
  pub upvals: Vec<*mut AstLocal>,
  pub loop_jumps: Vec<LoopJump>,
  pub loops: Vec<Loop>,
  pub inline_frames: Vec<InlineFrame>,
  pub captures: Vec<Capture>,
  pub exported_locals: Vec<*mut AstLocal>,
  pub exported_classes: Vec<(AstName, u8)>,
  pub class_locals: DenseHashMap<AstName, *mut AstLocal>,
}
