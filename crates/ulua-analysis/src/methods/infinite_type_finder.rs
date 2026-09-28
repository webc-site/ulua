//! `infinite_type_finder` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::records::ast_name::AstName;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type, get_type::get},
  records::{
    constraint_solver::ConstraintSolver,
    infinite_type_finder::InfiniteTypeFinder,
    instantiation_signature::InstantiationSignature,
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    pending_expansion_type::PendingExpansionType,
    scope::Scope,
  },
  type_aliases::{error_type::ErrorType, name_type::Name, type_id::TypeId},
};

impl InfiniteTypeFinder {
  pub fn infinite_type_finder_infinite_type_finder(
    solver: *mut ConstraintSolver,
    signature: &InstantiationSignature,
    scope: NonNull<Scope>,
  ) -> Self {
    let mut visitor = InfiniteTypeFinder {
      base: IterativeTypeVisitor {
        seen: DenseHashSet::default(),
        work_queue: Vec::new(),
        parent_cursor: -1,
        work_cursor: 0,
        visitor_name: String::from("InfiniteTypeFinder"),
        skip_bound_types: true,
        visit_once: true,
      },
      solver,
      signature: signature.clone(),
      scope,
      found_infinite_type: false,
    };
    visitor
      .base
      .iterative_type_visitor_string_bool_bool("InfiniteTypeFinder", true, true);
    visitor
  }
}

impl IterativeTypeVisitorTrait for InfiniteTypeFinder {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    InfiniteTypeFinder::visit_type_id(self, ty)
  }

  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    InfiniteTypeFinder::visit_type_id_pending_expansion_type(self, ty, petv)
  }
}

impl InfiniteTypeFinder {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found_infinite_type
  }

  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    if self.found_infinite_type {
      return false;
    }

    let name = ast_name_to_name(petv.name);
    let tf = unsafe {
      let scope = self.scope.as_ref();
      if let Some(prefix) = petv.prefix {
        scope.lookup_imported_type(&ast_name_to_name(prefix), &name)
      } else {
        scope.lookup_type(&name)
      }
    };

    let Some(tf) = tf else {
      return true;
    };

    if follow_type::follow(tf.r#type()) != follow_type::follow(self.signature.fn_sig.r#type()) {
      return true;
    }

    // 对齐 C++：比较前 follow 双方，任一侧为 ErrorType 时跳过该对
    // （error 不能作为「类型别名参数不同」的证据）。
    for (argument, parameter) in petv.type_arguments.iter().zip(tf.type_params()) {
      let pending_type_arg = follow_type::follow(*argument);
      let tf_type_param = follow_type::follow(parameter.ty);
      if get::<ErrorType>(pending_type_arg).is_some() || get::<ErrorType>(tf_type_param).is_some() {
        continue;
      }
      if pending_type_arg != tf_type_param {
        self.found_infinite_type = true;
        return false;
      }
    }

    for (argument, parameter) in petv.pack_arguments.iter().zip(tf.type_pack_params()) {
      if *argument != parameter.tp {
        self.found_infinite_type = true;
        return false;
      }
    }

    false
  }
}
fn ast_name_to_name(name: AstName) -> Name {
  // as_str_or_empty 已对 null 返回 ""，无需再判空
  name.as_str_or_empty().to_string()
}
