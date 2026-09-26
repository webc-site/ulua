use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{binding::Binding, global_types::GlobalTypes, symbol::Symbol},
};
pub fn try_get_global_binding(globals: &mut GlobalTypes, name: &str) -> Option<Binding> {
  // cpp: `globalNames.getOrAdd(name.c_str(), name.length())` —— 长度已显式给出，
  // `c_str()` 只为满足 `*const c_char` 形参；内部 `&str` 入口免去 CString 分配与指针往返。
  let ast_name = unsafe { (*(arc_as_mut(&globals.global_names.names))).get_or_add_str(name) };
  globals
    .global_scope
    .bindings
    .get(&Symbol::from_global(ast_name))
    .cloned()
}
