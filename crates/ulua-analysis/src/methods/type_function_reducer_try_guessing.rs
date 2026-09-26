use crate::{
  records::{
    arena_handle::Handle, type_function_reducer::TypeFunctionReducer,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub trait TypeFunctionGuessSubject: Copy {
  fn contains_in_guess_set(reducer: &TypeFunctionReducer, subject: Self) -> bool;
  fn guess_with(guesser: &mut TypeFunctionReductionGuesser, subject: Self) -> Option<Self>;
  fn replace_with(reducer: &mut TypeFunctionReducer, subject: Self, replacement: Self);
}

impl TypeFunctionGuessSubject for TypeId {
  fn contains_in_guess_set(reducer: &TypeFunctionReducer, subject: Self) -> bool {
    reducer.should_guess.contains(&(subject as *const ()))
  }

  fn guess_with(guesser: &mut TypeFunctionReductionGuesser, subject: Self) -> Option<Self> {
    guesser.guess_type_id(subject)
  }

  fn replace_with(reducer: &mut TypeFunctionReducer, subject: Self, replacement: Self) {
    reducer.replace_type_id(subject, replacement)
  }
}

impl TypeFunctionGuessSubject for TypePackId {
  fn contains_in_guess_set(reducer: &TypeFunctionReducer, subject: Self) -> bool {
    reducer.should_guess.contains(&(subject as *const ()))
  }

  fn guess_with(guesser: &mut TypeFunctionReductionGuesser, subject: Self) -> Option<Self> {
    guesser.guess_type_pack_id(subject)
  }

  fn replace_with(reducer: &mut TypeFunctionReducer, subject: Self, replacement: Self) {
    reducer.replace_type_pack_id(subject, replacement);
  }
}

impl TypeFunctionReducer {
  pub fn try_guessing<TID: TypeFunctionGuessSubject>(&mut self, subject: TID) -> bool {
    if TID::contains_in_guess_set(self, subject) {
      // ctx 是构造期以存活 `&mut` 借接线、类型编码非空的 Handle，只读取句柄字段。
      let ctx = self.ctx.get();
      let mut guesser =
                TypeFunctionReductionGuesser::type_function_reduction_guesser_type_function_reduction_guesser(
                    Handle::from_nonnull(ctx.arena),
                    Handle::from_nonnull(ctx.builtins),
                    ctx.normalizer.as_ptr(),
                );

      if let Some(guessed) = TID::guess_with(&mut guesser, subject) {
        TID::replace_with(self, subject, guessed);
        return true;
      }
    }

    false
  }
}
