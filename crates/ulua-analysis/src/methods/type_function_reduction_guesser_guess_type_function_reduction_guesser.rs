use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReductionGuesser {
  /// C++ `TypeFunctionReductionGuesser.cpp:96 guess(TypeId)`。
  pub fn guess_type_id(&mut self, typ: TypeId) -> Option<TypeId> {
    let guessed_type = self.guess_type(typ)?;

    let guess = follow_type_id(guessed_type);
    // C++: `if (get<TypeFunctionInstanceType>(guess)) return {};`
    if get_type_id::<TypeFunctionInstanceType>(guess).is_some() {
      return None;
    }

    Some(guess)
  }
}
