//! `TypeFunctionReducer::stepPack` (TypeFunction.cpp:624-646).
//!
//! BLOCKED on a cross-cluster record gap: `TypePackFunction::reducer` is
//! monomorphized as `ReducerFunction<TypeId>` (`fn(TypeId, Vec<TypeId>,
//! Vec<TypePackId>, *mut TypeFunctionContext) -> ...`), so it cannot be invoked
//! with the `TypePackId` `subject` that `stepPack` follows out of `queued_tps`
//! (`subject` is `*const TypePackVar`, the reducer's first param is
//! `*const Type`). The C++ `tfit->function->reducer(subject, ...)` needs the
//! reducer typed as `ReducerFunction<TypePackId>`. There are presently no type
//! pack functions (see `TypeFunctionReducer::get_state(TypePackId)` which always
//! returns `Unsolved`), so this branch is dead in practice; the body is left
//! unimplemented rather than faked or cast. Unblock by typing
//! `records/type_pack_function.rs::TypePackFunction::reducer` as
//! `ReducerFunction<TypePackId>`.
use crate::records::type_function_reducer::TypeFunctionReducer;

impl TypeFunctionReducer {
  pub fn step_pack(&mut self) {
    unimplemented!(
      "TypeFunctionReducer::stepPack: no type-pack functions exist (reducer is monomorphized to ReducerFunction<TypeId>); dead branch — see module doc to unblock"
    )
  }
}
