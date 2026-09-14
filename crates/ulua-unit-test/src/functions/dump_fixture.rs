use core::ffi::c_void;

use ulua_analysis::type_aliases::type_id::TypeId;
pub fn dump(name: &str, ty: TypeId) {
  let s = {
    // Simulate to_string(ty, {true}) by using the C++ type's string representation
    // Since we don't have direct access to to_string for TypeId in Rust yet,
    // we'll use a placeholder that matches the expected behavior
    let ty_ptr = ty as *const c_void;
    // In the actual C++ code, this would call to_string(ty, {true}).c_str()
    // For now, we'll create a debug representation
    format!("{:?}", ty_ptr)
  };

  // Use format_args! to match the printf("%s\t%s\n", ...) pattern
  println!("{}\t{}", name, s);
}
