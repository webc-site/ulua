use ulua_compiler::{
  functions::{
    set_compile_constant_boolean::set_compile_constant_boolean,
    set_compile_constant_nil::set_compile_constant_nil,
    set_compile_constant_number::set_compile_constant_number,
    set_compile_constant_string::set_compile_constant_string,
    set_compile_constant_vector::set_compile_constant_vector,
  },
  type_aliases::compile_constant::CompileConstant,
};
pub fn luau_library_constant_lookup(library: &str, member: &str, constant: CompileConstant) {
  if library == "vector" {
    if member == "zero" {
      set_compile_constant_vector(constant, 0.0, 0.0, 0.0, 0.0);
      return;
    }

    if member == "one" {
      set_compile_constant_vector(constant, 1.0, 1.0, 1.0, 0.0);
      return;
    }
  }

  if library == "Vector3" {
    if member == "one" {
      set_compile_constant_vector(constant, 1.0, 1.0, 1.0, 0.0);
      return;
    }

    if member == "xAxis" {
      set_compile_constant_vector(constant, 1.0, 0.0, 0.0, 0.0);
      return;
    }
  }

  if library == "test" {
    if member == "some_nil" {
      set_compile_constant_nil(constant);
      return;
    }

    if member == "some_boolean" {
      set_compile_constant_boolean(constant, true);
      return;
    }

    if member == "some_number" {
      set_compile_constant_number(constant, 4.75);
      return;
    }

    if member == "some_string" {
      set_compile_constant_string(constant, c"test".as_ptr(), 4);
    }
  }
}
