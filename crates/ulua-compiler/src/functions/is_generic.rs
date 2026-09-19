use ulua_ast::records::{ast_array::AstArray, ast_generic_type::AstGenericType, ast_name::AstName};

pub fn is_generic(name: AstName, generics: &AstArray<*mut AstGenericType>) -> bool {
  for gt in generics.as_slice() {
    let gt_ref = unsafe { &**gt };
    if gt_ref.name == name {
      return true;
    }
  }
  false
}
