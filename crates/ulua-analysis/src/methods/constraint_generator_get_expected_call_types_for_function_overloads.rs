use alloc::vec::Vec;

use crate::{
  functions::{
    begin_type::begin_intersection_type, flatten_type_pack::flatten_type_pack_id, follow_type,
    follow_type_pack, get_type, get_type_pack, reduce_union::reduce_union,
  },
  records::{
    constraint_generator::ConstraintGenerator, function_type::FunctionType,
    intersection_type::IntersectionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_id::TypeId,
};

impl ConstraintGenerator {
  pub fn get_expected_call_types_for_function_overloads(
    &mut self,
    fn_type: TypeId,
  ) -> Vec<Option<TypeId>> {
    let mut fun_tys: Vec<TypeId> = Vec::new();
    let followed_fn_type = follow_type::follow(fn_type);
    // 对照 C++:5253 `if (auto it = get<IntersectionType>(follow(fnType)))`
    if let Some(ity) = get_type::get::<IntersectionType>(followed_fn_type) {
      // C++ `for (TypeId intersectionComponent : it)`——迭代器展平嵌套
      // intersection 并 follow,裸遍历 parts 会漏掉嵌套成员。
      fun_tys.extend(begin_intersection_type(ity));
    }

    let mut expected_types: Vec<Option<TypeId>> = Vec::new();
    let overload_count = fun_tys.len();

    // 对照 C++:5265 `assignOption` lambda。
    let mut assign_option = |index: usize, ty: TypeId| {
      if index == expected_types.len() {
        expected_types.push(Some(ty));
        return;
      }

      let Some(el) = expected_types.get_mut(index) else {
        return;
      };
      let Some(existing) = *el else {
        *el = Some(ty);
        return;
      };

      let union = reduce_union(&[existing, ty]);
      if union.is_empty() {
        // SAFETY: builtin_types 为会话级裸指针，与 C++ 成员指针同契约。
        *el = Some({ self.builtin_types.get().never_type });
      } else {
        *el = Some(match union.as_slice() {
          [only] => *only,
          _ => self.make_union_vector_type_id(union),
        });
      }
    };

    for &overload in &fun_tys {
      let followed_overload = follow_type::follow(overload);
      // 对照 C++:5290 `if (const FunctionType* ftv = get<FunctionType>(follow(overload)))`
      let Some(ftv) = get_type::get::<FunctionType>(followed_overload) else {
        continue;
      };

      let (args_head, args_tail) = flatten_type_pack_id(ftv.arg_types);
      let start = if ftv.has_self { 1 } else { 0 };

      for (index, &arg) in args_head[start..].iter().enumerate() {
        assign_option(index, arg);
      }

      let Some(args_tail_id) = args_tail else {
        continue;
      };
      let tail = follow_type_pack::follow(args_tail_id);
      // 对照 C++:5297 `if (const VariadicTypePack* vtp = get<VariadicTypePack>(*argsTail))`
      if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tail) {
        for index in (args_head.len() - start)..overload_count {
          assign_option(index, vtp.ty);
        }
      }
    }

    expected_types
  }
}
