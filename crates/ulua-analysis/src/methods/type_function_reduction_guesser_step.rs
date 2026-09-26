use crate::{
  functions::{follow_type, get_type},
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReductionGuesser {
  /// C++ `TypeFunctionReductionGuesser.cpp:283 step()`。
  pub fn step(&mut self) {
    let t: TypeId = { *self.to_infer.front_mut() };
    self.to_infer.pop_front();
    let t = follow_type::follow(t);
    if let Some(tf) = get_type::get::<TypeFunctionInstanceType>(t) {
      // &T 隐式转 *const，命中清单外旧签名
      unsafe { self.infer_type_function_substitutions(t, tf) };
    }
  }
}
