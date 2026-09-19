use alloc::vec::Vec;
use core::ffi::c_char;

use ulua_ast::records::{
  ast_expr_function::AstExprFunction, ast_name::AstName, ast_stat_type_alias::AstStatTypeAlias,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::luau_bytecode_type::{LBC_TYPE_ANY, LBC_TYPE_FUNCTION, LBC_TYPE_TABLE},
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::functions::get_type::get_type;

/// cpp `getFunctionType`（Compiler.cpp）：返回 LBC_TYPE_* 原始字节序列。
/// Vec<u8> 与 cpp `std::string` 字节缓冲语义一致（type 字节 ≥ 128 时
/// String::push 的 UTF-8 编码会膨胀为两字节，故必须按字节写入）。
pub(crate) fn get_function_type(
  func: *const AstExprFunction,
  type_aliases: &DenseHashMap<AstName, *mut AstStatTypeAlias>,
  host_vector_type: *const c_char,
  userdata_types: &DenseHashMap<AstName, u8>,
  bytecode: &mut BytecodeBuilder,
) -> Vec<u8> {
  let func_ref = unsafe { &*func };
  let self_ = !func_ref.self_.is_null();

  let mut type_info: Vec<u8> = Vec::new();
  let args_size = func_ref.args.as_slice().len();
  type_info.reserve(args_size + (self_ as usize) + 2);

  type_info.push(LBC_TYPE_FUNCTION.0 as u8);
  type_info.push(((self_ as usize) + args_size) as u8);

  if self_ {
    type_info.push(LBC_TYPE_TABLE.0 as u8);
  }

  let mut have_non_any_param = false;
  for arg_ptr in func_ref.args.as_slice() {
    let arg = unsafe { &**arg_ptr };
    let mut seen_aliases = DenseHashSet::new(AstName::new());

    let ty = if !arg.annotation.is_null() {
      unsafe {
        get_type(
          arg.annotation,
          func_ref.generics,
          type_aliases,
          host_vector_type,
          userdata_types,
          bytecode,
          &mut seen_aliases,
        )
      }
    } else {
      LBC_TYPE_ANY
    };

    if ty != LBC_TYPE_ANY {
      have_non_any_param = true;
    }

    type_info.push(ty.0 as u8);
  }

  // If all parameters simplify to any, we can just omit type info for this function
  if !have_non_any_param {
    return Vec::new();
  }

  type_info
}
