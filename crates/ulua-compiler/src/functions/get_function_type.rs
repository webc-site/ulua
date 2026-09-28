use alloc::vec::Vec;

use ulua_ast::records::{
  ast_expr_function::AstExprFunction, ast_name::AstName, ast_stat_type_alias::AstStatTypeAlias,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::functions::{ast_slot_ref::ast_slot_ref, get_type::get_type};

/// cpp `getFunctionType`（Compiler.cpp）：返回 LBC_TYPE_* 原始字节序列。
/// Vec<u8> 与 cpp `std::string` 字节缓冲语义一致（type 字节 ≥ 128 时
/// String::push 的 UTF-8 编码会膨胀为两字节，故必须按字节写入）。
pub(crate) fn get_function_type(
  func_ref: &AstExprFunction,
  type_aliases: &DenseHashMap<AstName, *mut AstStatTypeAlias>,
  host_vector: Option<&[u8]>,
  userdata_types: &DenseHashMap<AstName, u8>,
  bytecode: &mut BytecodeBuilder,
) -> Vec<u8> {
  let self_ = func_ref.self_.is_some();

  let mut type_info: Vec<u8> = Vec::new();
  let args_size = func_ref.args.len();
  type_info.reserve(args_size + (self_ as usize) + 2);

  type_info.push(LuauBytecodeType::LBC_TYPE_FUNCTION.0 as u8);
  type_info.push(((self_ as usize) + args_size) as u8);

  if self_ {
    type_info.push(LuauBytecodeType::LBC_TYPE_TABLE.0 as u8);
  }

  let mut have_non_any_param = false;
  for arg in func_ref.args.iter_nodes() {
    // args 由 parser prepare_function_arguments 逐槽 push_local 分配，元素非空
    // 且 arena 存活；`iter_nodes` 只读物化，annotation 可空槽经 `ast_slot_ref`
    // 归一（None 由 get_type 化简为 ANY，与原 is_null 分支同语义）。
    let mut seen_aliases = DenseHashSet::default();

    let ty = get_type(
      ast_slot_ref(arg.annotation),
      &func_ref.generics,
      type_aliases,
      host_vector,
      userdata_types,
      bytecode,
      &mut seen_aliases,
    );

    if ty != LuauBytecodeType::LBC_TYPE_ANY {
      have_non_any_param = true;
    }

    type_info.push(ty.0 as u8);
  }

  // 所有形参都化简为 any 时，本函数可整体省略类型信息
  if !have_non_any_param {
    return Vec::new();
  }

  type_info
}
