use ulua_ast::records::ast_array::AstArray;

pub fn array<T>(items: &mut [T]) -> AstArray<T> {
  AstArray {
    data: items.as_mut_ptr(),
    size: items.len(),
  }
}
