use alloc::vec::Vec;

use crate::{
  functions::{flatten_type_pack::flatten_type_pack_id, follow_type, get_type},
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser, type_pack::TypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeFunctionReductionGuesser {
  /// C++ `TypeFunctionReductionGuesser.cpp:96 guess(TypeId)`。
  pub fn guess_type_id(&mut self, typ: TypeId) -> Option<TypeId> {
    let guessed_type = self.guess_type(typ)?;

    let guess = follow_type::follow(guessed_type);
    // C++: `if (get<TypeFunctionInstanceType>(guess)) return {};`
    if get_type::get::<TypeFunctionInstanceType>(guess).is_some() {
      return None;
    }

    Some(guess)
  }

  /// C++ `TypeFunctionReductionGuesser.cpp:110 guess(TypePackId)`。
  pub fn guess_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
    let (head, tail) = flatten_type_pack_id(tp);

    let mut guessed_head: Vec<TypeId> = Vec::with_capacity(head.len());

    for typ in head.iter().copied() {
      let guessed_type = self.guess_type(typ)?;

      // C++: `if (get<TypeFunctionInstanceType>(guess)) return {};`
      let guess = follow_type::follow(guessed_type);
      if get_type::get::<TypeFunctionInstanceType>(guess).is_some() {
        return None;
      }

      // C++: `guessedHead.push_back(*guessedType)`（存原始猜测值，非 follow 结果）
      guessed_head.push(guessed_type);
    }

    // SAFETY: arena 由构造方保证有效，同 C++ `arena->addTypePack`。
    Some(
      self
        .arena
        .get_mut()
        .add_type_pack_t(TypePack::new(guessed_head, tail)),
    )
  }
}
