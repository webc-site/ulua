//! 针对 CompileOptions 切片接口与常量设置切片接口的回归测试。

use core::{ffi::c_char, ptr::null};

use ulua_compiler::{
  functions::set_compile_constant::{set_compile_constant_slice, set_compile_constant_str},
  records::{compile_options::CompileOptions, constant::Constant},
  type_aliases::compile_constant::CompileConstant,
};

#[test]
fn compile_options_default_slice_accessors_return_none_or_empty() {
  let options = CompileOptions::default();

  assert_eq!(options.vector_lib(), None);
  assert_eq!(options.vector_ctor(), None);
  assert_eq!(options.vector_type(), None);
  assert_eq!(options.vector_lib_bytes(), None);
  assert_eq!(options.vector_ctor_bytes(), None);
  assert_eq!(options.vector_type_bytes(), None);

  assert_eq!(options.mutable_globals().count(), 0);
  assert_eq!(options.userdata_types().count(), 0);
  assert_eq!(options.libraries_with_known_members().count(), 0);
  assert_eq!(options.disabled_builtins().count(), 0);
}

#[test]
fn compile_options_slice_accessors_with_set_vector() {
  let mut options = CompileOptions::default();
  options.set_vector(Some(b"Vector3\0"), Some(b"new\0"), Some(b"Vector3\0"));

  assert_eq!(options.vector_lib(), Some("Vector3"));
  assert_eq!(options.vector_ctor(), Some("new"));
  assert_eq!(options.vector_type(), Some("Vector3"));

  assert_eq!(options.vector_lib_bytes(), Some(b"Vector3".as_slice()));
  assert_eq!(options.vector_ctor_bytes(), Some(b"new".as_slice()));
  assert_eq!(options.vector_type_bytes(), Some(b"Vector3".as_slice()));
}

#[test]
fn compile_options_slice_accessors_with_pointer_arrays() {
  let item1 = c"foo".as_ptr();
  let item2 = c"bar".as_ptr();
  let list: [*const c_char; 3] = [item1, item2, null()];

  let options = CompileOptions {
    mutable_globals: list.as_ptr(),
    ..Default::default()
  };

  let collected: Vec<&[u8]> = options.mutable_globals().collect();
  assert_eq!(collected, vec![b"foo".as_slice(), b"bar".as_slice()]);
}

#[test]
fn set_compile_constant_slice_and_str() {
  let mut slot = Constant::default();
  let ptr = &mut slot as *mut Constant as CompileConstant;

  set_compile_constant_slice(ptr, b"hello world");
  assert_eq!(slot.get_string_bytes(), b"hello world");

  set_compile_constant_str(ptr, "native slice");
  assert_eq!(slot.get_string_bytes(), b"native slice");
}

#[test]
fn compile_options_safe_builder_and_const_construction() {
  const OPTS: CompileOptions = CompileOptions::new()
    .with_optimization_level(2)
    .with_debug_level(0)
    .with_type_info_level(1)
    .with_coverage_level(2)
    .with_vector(Some(b"Vector3\0"), Some(b"new\0"), Some(b"Vector3\0"));

  assert_eq!(OPTS.optimization_level, 2);
  assert_eq!(OPTS.debug_level, 0);
  assert_eq!(OPTS.type_info_level, 1);
  assert_eq!(OPTS.coverage_level, 2);
  assert_eq!(OPTS.vector_lib(), Some("Vector3"));
  assert_eq!(OPTS.vector_ctor(), Some("new"));
  assert_eq!(OPTS.vector_type(), Some("Vector3"));
}

#[test]
fn compile_options_native_compilation_builder() {
  let opts = CompileOptions::new().with_native_compilation();
  assert_eq!(opts.optimization_level, 2);
  assert_eq!(opts.type_info_level, 1);
  assert_eq!(opts.debug_level, 1);
  assert_eq!(opts.coverage_level, 0);
}

#[test]
fn compile_options_list_builders() {
  let item1 = c"foo".as_ptr();
  let list: [*const c_char; 2] = [item1, null()];

  let opts = CompileOptions::new()
    .with_mutable_globals(list.as_ptr())
    .with_userdata_types(list.as_ptr())
    .with_libraries_with_known_members(list.as_ptr())
    .with_disabled_builtins(list.as_ptr());

  assert_eq!(
    opts.mutable_globals().collect::<Vec<_>>(),
    vec![b"foo".as_slice()]
  );
  assert_eq!(
    opts.userdata_types().collect::<Vec<_>>(),
    vec![b"foo".as_slice()]
  );
  assert_eq!(
    opts.libraries_with_known_members().collect::<Vec<_>>(),
    vec![b"foo".as_slice()]
  );
  assert_eq!(
    opts.disabled_builtins().collect::<Vec<_>>(),
    vec![b"foo".as_slice()]
  );
}

#[test]
fn compile_options_individual_vector_builders() {
  let opts = CompileOptions::new()
    .with_vector_lib(Some(b"Lib\0"))
    .with_vector_ctor(Some(b"ctor\0"))
    .with_vector_type(Some(b"Type\0"));

  assert_eq!(opts.vector_lib(), Some("Lib"));
  assert_eq!(opts.vector_ctor(), Some("ctor"));
  assert_eq!(opts.vector_type(), Some("Type"));
}
