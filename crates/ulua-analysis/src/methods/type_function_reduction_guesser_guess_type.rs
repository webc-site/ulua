use crate::{
  functions::{follow_type, follow_type_utils::follow_optional_ty, get_type},
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};
impl TypeFunctionReductionGuesser {
  pub fn guess_type(&mut self, arg: TypeId) -> Option<TypeId> {
    let t = follow_type::follow(arg);

    if self.substitutable.contains(&t) {
      // 上方 `contains(&t)` 判定蕴含 find 命中，双写合一直接绑定非空句柄。
      let subst0 = self
        .substitutable
        .find(&t)
        .copied()
        .expect("上方 contains 判定蕴含 find 命中");
      let subst = follow_optional_ty(Some(subst0)).unwrap_or(subst0);
      if subst == t
        || self.substitutable.contains(&subst)
        || get_type::get::<TypeFunctionInstanceType>(subst).is_none()
      {
        return Some(subst);
      } else {
        return self.guess_type(subst);
      }
    }

    if get_type::get::<TypeFunctionInstanceType>(t).is_some()
      && self.function_reduces_to.contains(&t)
    {
      return self.function_reduces_to.find(&t).copied();
    }

    None
  }
}
