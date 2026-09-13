use crate::{
  functions::{
    follow_type::follow_type_id, follow_type_utils::follow_optional_ty, get_type_alt_j::get_type_id,
  },
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};
impl TypeFunctionReductionGuesser {
  pub fn guess_type(&mut self, arg: TypeId) -> Option<TypeId> {
    let t = follow_type_id(arg);

    if self.substitutable.contains(&t) {
      let subst_opt = self.substitutable.find(&t).copied();
      let subst = follow_optional_ty(subst_opt).unwrap_or(subst_opt.unwrap());
      if subst == t
        || self.substitutable.contains(&subst)
        || get_type_id::<TypeFunctionInstanceType>(subst).is_none()
      {
        return Some(subst);
      } else {
        return self.guess_type(subst);
      }
    }

    if !get_type_id::<TypeFunctionInstanceType>(t).is_none()
      && self.function_reduces_to.contains(&t)
    {
      return self.function_reduces_to.find(&t).copied();
    }

    None
  }
}
