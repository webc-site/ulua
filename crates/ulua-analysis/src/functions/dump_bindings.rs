use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
  records::{scope::Scope, to_string_options::ToStringOptions},
};

/// # Safety
/// 调用方须保证 `scope` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
/// C++ `static void dumpBindings(NotNull<Scope> scope, ToStringOptions& opts)`.
pub unsafe fn dump_bindings(scope: *mut Scope, opts: &mut ToStringOptions) {
  let scope_ref = unsafe { &*scope };

  for (k, v) in &scope_ref.bindings {
    let d: String = to_string_type_id_to_string_options(v.type_id, opts);
    let key_str = k.name();
    println!("\t{} : {}", key_str, d);
  }

  for child in &scope_ref.children {
    unsafe { dump_bindings(*child, opts) };
  }
}
