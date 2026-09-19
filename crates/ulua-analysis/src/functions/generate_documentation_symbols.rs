use alloc::string::String;

use crate::{
  functions::{as_mutable_type::as_mutable_type_id, get_mutable_type::get_mutable_type_id},
  records::{extern_type::ExternType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
pub unsafe fn generate_documentation_symbols(ty: TypeId, root_name: String) {
  if unsafe { (*ty).persistent } {
    return;
  }

  let mutable_type_ptr = as_mutable_type_id(ty);
  if mutable_type_ptr.is_null() {
    return;
  }

  // SAFETY: mutable_type_ptr 非空已检查，文档符号写回同 C++
  unsafe {
    (*mutable_type_ptr).documentation_symbol = Some(root_name.clone());
  }

  if let Some(table_type) = get_mutable_type_id::<TableType>(ty) {
    for (name, prop) in &mut table_type.props {
      let mut n = String::with_capacity(root_name.len() + 1 + name.len());
      n.push_str(&root_name);
      n.push('.');
      n.push_str(name);
      prop.documentation_symbol = Some(n);
    }
  } else if let Some(extern_type) = get_mutable_type_id::<ExternType>(ty) {
    for (name, prop) in &mut extern_type.props {
      let mut n = String::with_capacity(root_name.len() + 1 + name.len());
      n.push_str(&root_name);
      n.push('.');
      n.push_str(name);
      prop.documentation_symbol = Some(n);
    }
  }
}
