use core::{
  mem::size_of_val,
  ptr::{copy_nonoverlapping, null_mut},
};

use ulua_ast::records::ast_stat::AstStat;

use crate::records::reducer::Reducer;

impl Reducer {
  pub fn reallocate_statements(&mut self, statements: &[*mut AstStat]) -> *mut *mut AstStat {
    let count = statements.len();
    if count == 0 {
      return null_mut();
    }

    let bytes = size_of_val(statements);
    let new_data = self.allocator.allocate(bytes) as *mut *mut AstStat;

    unsafe {
      copy_nonoverlapping(statements.as_ptr(), new_data, count);
    }

    new_data
  }
}
