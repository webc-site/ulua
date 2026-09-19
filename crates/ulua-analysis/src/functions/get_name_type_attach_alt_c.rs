use core::ffi::c_void;
extern crate alloc;

use alloc::string::ToString;
use core::ffi::c_char;

use ulua_ast::records::allocator::Allocator;

use crate::{
  functions::{
    allocate_string_type_attach::allocate_string_luau_allocator_string_view,
    generate_name::generate_name,
  },
  records::generic_type_pack::GenericTypePack,
  type_aliases::synthetic_names::SyntheticNames,
};

pub fn get_name_allocator_synthetic_names_generic_type_pack(
  allocator: &mut Allocator,
  synthetic_names: &mut SyntheticNames,
  r#gen: &GenericTypePack,
) -> *mut c_char {
  let s = synthetic_names.size();
  let n_ptr = synthetic_names.get_or_insert(r#gen as *const GenericTypePack as *const c_void);

  if (*n_ptr).is_null() {
    let name = if r#gen.explicit_name {
      r#gen.name.to_string()
    } else {
      generate_name(s)
    };

    // cpp `allocate(allocator, name)`：分配 + 拷贝 + NUL 结尾由共用助手收口
    *n_ptr = allocate_string_luau_allocator_string_view(allocator, &name);
  }
  *n_ptr
}
