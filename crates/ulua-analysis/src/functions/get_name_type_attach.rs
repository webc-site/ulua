use alloc::string::ToString;

use ulua_ast::records::allocator::Allocator;

use crate::{
  functions::{alloc_nul_string::alloc_nul_string, generate_name::generate_name},
  records::{generic_type::GenericType, generic_type_pack::GenericTypePack},
  type_aliases::synthetic_names::SyntheticNames,
};

pub fn get_name_allocator_synthetic_names_generic_type(
  allocator: &mut Allocator,
  synthetic_names: &mut SyntheticNames,
  r#gen: &GenericType,
) -> *mut u8 {
  let s = synthetic_names.size();
  let n_ptr = synthetic_names.get_or_insert(r#gen as *const GenericType as *const ());

  if (*n_ptr).is_null() {
    let name = if r#gen.explicit_name {
      r#gen.name.to_string()
    } else {
      generate_name(s)
    };

    // cpp `allocate(allocator, name)`：分配 + 拷贝 + NUL 结尾由共用助手收口
    *n_ptr = alloc_nul_string(allocator, &name);
  }
  *n_ptr
}

pub fn get_name_allocator_synthetic_names_generic_type_pack(
  allocator: &mut Allocator,
  synthetic_names: &mut SyntheticNames,
  r#gen: &GenericTypePack,
) -> *mut u8 {
  let s = synthetic_names.size();
  let n_ptr = synthetic_names.get_or_insert(r#gen as *const GenericTypePack as *const ());

  if (*n_ptr).is_null() {
    let name = if r#gen.explicit_name {
      r#gen.name.to_string()
    } else {
      generate_name(s)
    };

    // cpp `allocate(allocator, name)`：分配 + 拷贝 + NUL 结尾由共用助手收口
    *n_ptr = alloc_nul_string(allocator, &name);
  }
  *n_ptr
}
