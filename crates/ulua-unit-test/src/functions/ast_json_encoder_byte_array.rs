use ulua_ast::records::ast_array::AstArray;

/// 批 2 存储面：cpp `AstArray<char>` 测试夹具改 `AstArray<u8>`（同字节域，
/// 原 `c_char_array` 名随存储类型归一为 `byte_array`）。
pub fn byte_array(items: &mut [u8]) -> AstArray<u8> {
  AstArray {
    data: items.as_mut_ptr(),
    size: items.len(),
  }
}
