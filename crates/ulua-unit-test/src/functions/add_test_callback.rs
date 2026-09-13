use crate::{
  functions::get_register_callbacks::get_register_callbacks,
  type_aliases::register_callback::RegisterCallback,
};

pub fn add_test_callback(cb: RegisterCallback) -> i32 {
  get_register_callbacks().insert(cb);
  0
}
