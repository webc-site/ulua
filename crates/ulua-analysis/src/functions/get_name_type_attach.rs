use alloc::string::ToString;
use core::{
  ffi::{c_char, c_void},
  ptr::copy_nonoverlapping,
};

use ulua_ast::records::allocator::Allocator;

use crate::{
  functions::generate_name::generate_name, records::generic_type::GenericType,
  type_aliases::synthetic_names::SyntheticNames,
};
pub fn get_name_allocator_synthetic_names_generic_type(
  allocator: &mut Allocator,
  synthetic_names: &mut SyntheticNames,
  r#gen: &GenericType,
) -> *mut c_char {
  let s = synthetic_names.size();
  let n_ptr = synthetic_names.get_or_insert(r#gen as *const GenericType as *const c_void);

  unsafe {
    if (*n_ptr).is_null() {
      let str = if r#gen.explicit_name {
        r#gen.name.to_string()
      } else {
        generate_name(s)
      };

      let size = str.len();
      let n = allocator.allocate(size + 1) as *mut c_char;
      copy_nonoverlapping(str.as_ptr() as *const c_char, n, size);
      *n.add(size) = 0;
      *n_ptr = n;
    }
    *n_ptr
  }
}
