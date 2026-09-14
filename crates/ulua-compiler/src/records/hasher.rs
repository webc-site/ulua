use core::hash::{Hash, Hasher as CoreHasher};

use ulua_ast::records::{ast_expr_table::AstExprTable, ast_name::AstName};
use ulua_common::records::dense_hash_table::DenseHasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Hasher;

impl DenseHasher<(*mut AstExprTable, AstName)> for Hasher {
  fn hash(&self, key: &(*mut AstExprTable, AstName)) -> usize {
    struct FnvHasher(u64);

    impl CoreHasher for FnvHasher {
      fn finish(&self) -> u64 {
        self.0
      }

      fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
          self.0 ^= byte as u64;
          self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
      }
    }

    let mut hasher = FnvHasher(0xcbf2_9ce4_8422_2325);
    key.hash(&mut hasher);
    hasher.finish() as usize
  }
}
