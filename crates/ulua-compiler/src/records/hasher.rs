use core::mem::size_of;

use ulua_ast::records::{ast_expr_table::AstExprTable, ast_name::AstName};
use ulua_common::records::dense_hash_table::DenseHasher;

/// museair 固定种子：键为指针地址（本就随 ASLR 变化），仅求分布稳定，非输出契约。
const SEED: u64 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Hasher;

impl DenseHasher<(*mut AstExprTable, AstName)> for Hasher {
  fn hash(&self, key: &(*mut AstExprTable, AstName)) -> usize {
    // 键为两个指针（table 指针 + AstName 内部字符串指针），按小端序列进栈上数组，一次成型。
    const PTR_BYTES: usize = size_of::<usize>();
    let mut bytes = [0u8; PTR_BYTES * 2];
    bytes[..PTR_BYTES].copy_from_slice(&key.0.addr().to_le_bytes());
    bytes[PTR_BYTES..].copy_from_slice(&key.1.value.addr().to_le_bytes());
    museair::hash(&bytes, SEED) as usize
  }
}
