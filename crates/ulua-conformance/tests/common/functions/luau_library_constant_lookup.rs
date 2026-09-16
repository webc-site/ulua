use core::ffi::c_void;

use ulua_compiler::{
  functions::{
    luau_set_compile_constant_boolean::luau_set_compile_constant_boolean,
    luau_set_compile_constant_nil::luau_set_compile_constant_nil,
    luau_set_compile_constant_number::luau_set_compile_constant_number,
    luau_set_compile_constant_string::luau_set_compile_constant_string,
    luau_set_compile_constant_vector::luau_set_compile_constant_vector,
    set_compile_constant_vector::set_compile_constant_vector,
  },
  type_aliases::compile_constant::CompileConstant,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn luau_library_constant_lookup(
  library: &str,
  member: &str,
  constant: *mut CompileConstant,
) {
  let const_ptr = constant as *mut c_void;

  if library == "vector" {
    match member {
      "zero" => {
        set_compile_constant_vector(const_ptr, 0.0, 0.0, 0.0, 0.0);
        return;
      }
      "one" => {
        set_compile_constant_vector(const_ptr, 1.0, 1.0, 1.0, 0.0);
        return;
      }
      _ => {}
    }
  }

  if library == "Vector3" {
    match member {
      "xAxis" => {
        set_compile_constant_vector(const_ptr, 1.0, 0.0, 0.0, 0.0);
        return;
      }
      "yAxis" => {
        set_compile_constant_vector(const_ptr, 0.0, 1.0, 0.0, 0.0);
        return;
      }
      _ => {}
    }
  }

  if library == "test" {
    match member {
      "some_nil" => {
        luau_set_compile_constant_nil(const_ptr);
      }
      "some_boolean" => {
        luau_set_compile_constant_boolean(const_ptr, true);
      }
      "some_number" => {
        unsafe { luau_set_compile_constant_number(constant, 4.75) };
      }
      "some_vector" => {
        luau_set_compile_constant_vector(const_ptr, 1.0, 2.0, 4.0, 8.0);
      }
      "some_string" => {
        let s = c"test".as_ptr();
        luau_set_compile_constant_string(const_ptr, s, 4);
      }
      _ => {}
    }
  }
}
