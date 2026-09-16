use alloc::vec::Vec;

use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id, follow_type_pack,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id, reduce_union::reduce_union,
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
    let followed_fn_type = follow_type_id(fn_type);
    // 对照 C++:5253 `if (auto it = get<IntersectionType>(follow(fnType)))`
    if let Some(ity) = get_type_id::<IntersectionType>(followed_fn_type) {
      for &intersection_component in &ity.parts {
        fun_tys.push(intersection_component);
      }
    }

    let mut expected_types: Vec<Option<TypeId>> = Vec::new();

    let mut assign_option = |index: usize, ty: TypeId, expected_types: &mut Vec<Option<TypeId>>| {
      if index == expected_types.len() {
        expected_types.push(Some(ty));
      } else if let Some(el) = expected_types.get_mut(index) {
        if let Some(existing) = *el {
          let result = reduce_union(&[existing, ty]);
          if result.is_empty() {
            // SAFETY: builtin_types 为会话级裸指针句柄，与 C++ 成员指针同契约。
            *el = Some(unsafe { (*self.builtin_types).never_type });
          } else if result.len() == 1 {
            *el = Some(result[0]);
          } else {
            *el = Some(self.make_union_vector_type_id(result));
          }
        } else {
          *el = Some(ty);
        }
      }
    };

    for &overload in &fun_tys {
      let followed_overload = follow_type_id(overload);
      // 对照 C++:5290 `if (const FunctionType* ftv = get<FunctionType>(follow(overload)))`
      if let Some(ftv) = get_type_id::<FunctionType>(followed_overload) {
        let (args_head, args_tail) = flatten_type_pack_id(ftv.arg_types);

        let start = if ftv.has_self { 1 } else { 0 };
        let mut index = 0;

        for &arg in &args_head[start..] {
          assign_option(index, arg, &mut expected_types);
          index += 1;
        }

        if let Some(args_tail_id) = args_tail {
          let tail = unsafe { follow_type_pack::follow(args_tail_id) };
          // 对照 C++:5297 `if (const VariadicTypePack* vtp = get<VariadicTypePack>(*argsTail))`
          if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tail) {
            while index < fun_tys.len() {
              assign_option(index, vtp.ty, &mut expected_types);
              index += 1;
            }
          }
        }
      }
    }

    expected_types
  }
}
