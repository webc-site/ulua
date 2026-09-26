//! 针对 CompileOptions 切片接口与常量设置切片接口的回归测试。

use core::{ffi::c_char, ptr::null};

use ulua_compiler::{
  functions::set_compile_constant::{set_compile_constant_slice, set_compile_constant_str},
  records::{compile_options::CompileOptions, constant::Constant},
  type_aliases::{
    compile_constant::CompileConstant, library_member_type_callback::LibraryMemberTypeCallback,
  },
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
fn library_member_type_callback_i32_signature() {
  unsafe extern "C-unwind" fn mock_callback(_library: *const u8, _member: *const u8) -> i32 {
    42
  }

  let cb: LibraryMemberTypeCallback = Some(mock_callback);
  let res = cb.map(|f| unsafe { f(null(), null()) });
  assert_eq!(res, Some(42));
}
