use core::ptr::{NonNull, null};

use ulua_ast::{
  records::{ast_name::AstName, ast_stat_block::AstStatBlock},
  visit::AstVisitable,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    constraint_generator::ConstraintGenerator, global_prepopulator::GlobalPrepopulator,
    scope::Scope,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn prepopulate_global_scope(
    &mut self,
    global_scope: &ScopePtr,
    program: *mut AstStatBlock,
  ) {
    let global_scope_raw = global_scope.as_ref() as *const Scope as *mut Scope;
    let mut gp = GlobalPrepopulator {
      global_scope: unsafe { NonNull::new_unchecked(global_scope_raw) },
      arena: unsafe { NonNull::new_unchecked(self.arena) },
      dfg: unsafe { NonNull::new_unchecked(self.dfg as *mut _) },
      uninitialized_globals: DenseHashSet::new(AstName { value: null() }),
    };

    if let Some(module) = &self.module {
      (self.prepare_module_scope)(&module.name, global_scope);
    }

    unsafe {
      (*program).visit(&mut gp);
    }

    for name in gp.uninitialized_globals.iter() {
      self.uninitialized_globals.insert(name);
    }

    let root_scope_raw =
      unsafe { (*self.type_function_runtime).root_scope.as_ref() as *const Scope as *mut Scope };

    let mut tfgp = GlobalPrepopulator {
      global_scope: unsafe { NonNull::new_unchecked(root_scope_raw) },
      arena: unsafe { NonNull::new_unchecked(self.arena) },
      dfg: unsafe { NonNull::new_unchecked(self.dfg as *mut _) },
      uninitialized_globals: DenseHashSet::new(AstName { value: null() }),
    };

    unsafe {
      (*program).visit(&mut tfgp);
    }

    for name in tfgp.uninitialized_globals.iter() {
      self.uninitialized_globals.insert(name);
    }
  }
}
