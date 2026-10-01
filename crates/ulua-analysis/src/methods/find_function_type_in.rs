//! `find_function_type_in` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};
use core::ptr::{from_ref, null};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{get_type, size_type_pack::size},
  records::{
    arena_handle::alias_ref,
    find_function_type_in::FindFunctionTypeIn,
    function_type::FunctionType,
    intersection_type::IntersectionType,
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl FindFunctionTypeIn {
  pub fn new(number_of_lambda_parameters: i32) -> Self {
    let mut visitor = FindFunctionTypeIn {
      base: IterativeTypeVisitor {
        seen: DenseHashSet::default(),
        work_queue: Vec::new(),
        parent_cursor: -1,
        work_cursor: 0,
        visitor_name: String::from("FindFunctionTypeIn"),
        skip_bound_types: true,
        visit_once: true,
      },
      number_of_lambda_parameters,
      candidate: null(),
    };
    visitor
      .base
      .iterative_type_visitor_string_bool_bool("FindFunctionTypeIn", true, true);
    visitor
  }
}

impl IterativeTypeVisitorTrait for FindFunctionTypeIn {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    FindFunctionTypeIn::visit_type_id(self, ty)
  }

  fn visit_type_id_union_type(&mut self, ty: TypeId, utv: &UnionType) -> bool {
    FindFunctionTypeIn::visit_type_id_union_type(self, ty, utv)
  }

  fn visit_type_id_intersection_type(&mut self, ty: TypeId, itv: &IntersectionType) -> bool {
    FindFunctionTypeIn::visit_type_id_intersection_type(self, ty, itv)
  }

  fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    FindFunctionTypeIn::visit_type_id_function_type(self, ty, ftv)
  }
}

impl FindFunctionTypeIn {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    false
  }

  pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _utv: &UnionType) -> bool {
    true
  }

  pub fn visit_type_id_intersection_type(&mut self, _ty: TypeId, _itv: &IntersectionType) -> bool {
    true
  }

  pub fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    // This logic is a little clowny.
    //
    // For bidirectional inference we're trying to _guess_ what the user
    // is intending so that we can give decent results. For functions, we
    // will error if the user doesn't provide exactly the correct number of
    // arguments.
    //
    // The original C++ implementation attempts to prefer candidates with
    // an arg count closest to the lambda parameter count.
    let candidate = self.candidate;

    if candidate.is_null()
      // Safety: `||` 短路保证 candidate 非空才进入本调用；candidate 只由
      // get_type::get::<FunctionType> 的 arena 存活结果写入（type arena 活过整个
      // 推断回调），candidate_arg_count 契约（指向存活 FunctionType）满足，
      // 其内仅只读 arg_types 句柄字段。
      || (unsafe { candidate_arg_count(candidate) } as i32 - self.number_of_lambda_parameters).abs()
        > (ftv_arg_count(ftv) as i32 - self.number_of_lambda_parameters).abs()
    {
      self.candidate = get_type::get::<FunctionType>(ty).map_or(null(), from_ref);
      return false;
    }

    false
  }
}
/// 读取候选 FunctionType 的实参包长度。
///
/// # Safety
/// `candidate` 必须指向存活的 FunctionType：调用点仅从
/// `get_type::get::<FunctionType>`（类型 arena 持有、活过整个字面量推断回调）
/// 的结果写入，且调用前已判非空。
unsafe fn candidate_arg_count(candidate: *const FunctionType) -> usize {
  type_pack_len(alias_ref(candidate).arg_types)
}
fn ftv_arg_count(ftv: &FunctionType) -> usize {
  type_pack_len(ftv.arg_types)
}
fn type_pack_len(arg_types: TypePackId) -> usize {
  // C++ uses `size(argTypes)` (TypePack.cpp:308) — count the bound head types
  // and follow the tail. The default-log overload is `None`（C++ `log = nullptr`）。
  size(arg_types, None)
}
