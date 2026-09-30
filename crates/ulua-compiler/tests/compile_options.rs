//! 针对 CompileOptions 惯用 Rust 字段接口与常量设置切片接口的回归测试。

use ulua_compiler::{
  functions::set_compile_constant::{set_compile_constant_slice, set_compile_constant_str},
  records::{compile_options::CompileOptions, constant::Constant},
  type_aliases::compile_constant::CompileConstant,
};

#[test]
fn compile_options_default_accessors_return_none_or_empty() {
  let options = CompileOptions::default();

  assert_eq!(options.vector_lib, None);
  assert_eq!(options.vector_ctor, None);
  assert_eq!(options.vector_type, None);

  assert!(options.mutable_globals.is_empty());
  assert!(options.userdata_types.is_empty());
  assert!(options.libraries_with_known_members.is_empty());
  assert!(options.disabled_builtins.is_empty());
}

#[test]
fn compile_options_accessors_with_set_vector() {
  let options = CompileOptions::new().with_vector(Some("Vector3"), Some("new"), Some("Vector3"));

  assert_eq!(options.vector_lib.as_deref(), Some("Vector3"));
  assert_eq!(options.vector_ctor.as_deref(), Some("new"));
  assert_eq!(options.vector_type.as_deref(), Some("Vector3"));
}

#[test]
fn compile_options_list_fields_round_trip() {
  let options = CompileOptions::default()
    .with_mutable_globals(["foo", "bar"])
    .with_userdata_types(["foo"])
    .with_libraries_with_known_members(["foo"])
    .with_disabled_builtins(["foo"]);

  assert_eq!(
    options.mutable_globals,
    ["foo".to_owned(), "bar".to_owned()]
  );
  assert_eq!(options.userdata_types, ["foo".to_owned()]);
  assert_eq!(options.libraries_with_known_members, ["foo".to_owned()]);
  assert_eq!(options.disabled_builtins, ["foo".to_owned()]);
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
fn compile_options_safe_builder() {
  let opts = CompileOptions::new()
    .with_optimization_level(2)
    .with_debug_level(0)
    .with_type_info_level(1)
    .with_coverage_level(2)
    .with_vector(Some("Vector3"), Some("new"), Some("Vector3"));

  assert_eq!(opts.optimization_level, 2);
  assert_eq!(opts.debug_level, 0);
  assert_eq!(opts.type_info_level, 1);
  assert_eq!(opts.coverage_level, 2);
  assert_eq!(opts.vector_lib.as_deref(), Some("Vector3"));
  assert_eq!(opts.vector_ctor.as_deref(), Some("new"));
  assert_eq!(opts.vector_type.as_deref(), Some("Vector3"));
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
fn compile_options_individual_vector_builders() {
  let opts = CompileOptions::new()
    .with_vector_lib(Some("Lib"))
    .with_vector_ctor(Some("ctor"))
    .with_vector_type(Some("Type"));

  assert_eq!(opts.vector_lib.as_deref(), Some("Lib"));
  assert_eq!(opts.vector_ctor.as_deref(), Some("ctor"));
  assert_eq!(opts.vector_type.as_deref(), Some("Type"));
}
