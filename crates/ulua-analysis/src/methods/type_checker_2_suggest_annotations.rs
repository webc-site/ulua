//! `TypeChecker2::suggestAnnotations`（TypeChecker2.cpp:4273-4314）。
use alloc::collections::VecDeque;
use core::ptr::null;

use ulua_ast::records::ast_expr_function::AstExprFunction;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    get_type_alt_j::get_type_id,
  },
  records::{
    explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended,
    function_type::FunctionType, intersection_type::IntersectionType, normalizer::Normalizer,
    type_arena::TypeArena, type_checker_2::TypeChecker2,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{type_error_data::IntoTypeErrorData, type_id::TypeId},
};
impl TypeChecker2 {
  pub fn type_checker_2_suggest_annotations(&mut self, expr: &AstExprFunction, ty: TypeId) {
    // C++ LUAU_ASSERT(inferredFtv)（TypeChecker2.cpp:4275-4276）：调用方保证传入
    // 归一化函数类型的 parts.front()，必为 FunctionType。
    let inferred_ftv = get_type_id::<FunctionType>(ty).expect("inferred type is a function");

    let mut work_list: VecDeque<TypeId> = VecDeque::new();
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());

    // SAFETY: self.module 与类型检查会话同寿；guesser 持裸指针的旧接口暂保持。
    let mut guesser =
      TypeFunctionReductionGuesser::type_function_reduction_guesser_type_function_reduction_guesser(
        unsafe { &mut (*self.module).internal_types as *mut TypeArena },
        self.builtin_types,
        &mut self.normalizer as *mut Normalizer,
      );

    let (ret_head, _ret_tail) = flatten_type_pack_id(inferred_ftv.ret_types);
    work_list.extend(ret_head);

    while let Some(front) = work_list.pop_front() {
      let t = follow_type_id(front);

      if seen.contains(&t) {
        continue;
      }
      seen.insert(t);

      if let Some(ut) = get_type_id::<UnionType>(t) {
        work_list.extend(ut.options.iter().copied());
        continue;
      }

      if let Some(it) = get_type_id::<IntersectionType>(t) {
        work_list.extend(it.parts.iter().copied());
        continue;
      }

      if get_type_id::<TypeFunctionInstanceType>(t).is_some() {
        let result = guesser.guess_type_function_reduction_for_function_expr(expr, inferred_ftv, t);
        if result.should_recommend_annotation
          && get_type_id::<UnknownType>(result.guessed_return_type).is_none()
        {
          let err = ExplicitFunctionAnnotationRecommended {
            recommended_args: result.guessed_function_annotations,
            recommended_return: result.guessed_return_type,
          };
          let location = expr.base.base.location;
          self.report_error_type_error_data_location(err.into_type_error_data(), &location);
        }
      }
    }
  }
}
