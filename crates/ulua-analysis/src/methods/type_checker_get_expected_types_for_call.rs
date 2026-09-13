use alloc::vec::Vec;

use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type_pack::follow_type_pack_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id, reduce_union::reduce_union,
  },
  records::{
    demoter::Demoter, function_type::FunctionType, type_checker::TypeChecker,
    union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_id::TypeId,
};
impl TypeChecker {
  pub fn get_expected_types_for_call(
    &mut self,
    overloads: &Vec<TypeId>,
    argument_count: usize,
    self_call: bool,
  ) -> Vec<Option<TypeId>> {
    let mut expected_types: Vec<Option<TypeId>> = Vec::new();

    let mut assign_option = |index: usize, ty: TypeId| {
      if index == expected_types.len() {
        expected_types.push(Some(ty));
      } else if let Some(el) = expected_types.get_mut(index) {
        if let Some(existing) = *el {
          let result = reduce_union(&[existing, ty]);
          if result.is_empty() {
            *el = Some(self.never_type);
          } else if result.len() == 1 {
            *el = Some(result[0]);
          } else {
            *el = Some(self.add_type(&UnionType { options: result }));
          }
        } else {
          *el = Some(ty);
        }
      }
    };

    for &overload in overloads {
      if let Some(ftv) = get_type_id::<FunctionType>(overload) {
        let (args_head, args_tail) = flatten_type_pack_id(ftv.arg_types);

        let start = if self_call { 1 } else { 0 };

        // 对齐 C++ 的 `for (i = start; i < size)`：head 不足 start 时零次迭代
        for (index, &arg) in args_head.iter().skip(start).enumerate() {
          assign_option(index, arg);
        }

        if let Some(tail) = args_tail {
          let tail = unsafe { follow_type_pack_id(tail) };
          if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tail) {
            let mut index = args_head.len().saturating_sub(start);
            while index < argument_count {
              assign_option(index, vtp.ty);
              index += 1;
            }
          }
        }
      }
    }

    let mut demoter = Demoter {
      arena: self.normalizer.arena,
      builtins: self.builtin_types,
    };
    demoter.demote(&mut expected_types);

    expected_types
  }
}
